use std;
use std::collections::HashMap;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::io::{self, Cursor, Read};
use std::fs::File;

use lru_cache::LruCache;
use pathfinder_font_renderer::{FontContext, FontInstance, GlyphImage};
use pathfinder_font_renderer::{GlyphKey, SubpixelOffset};
use pathfinder_partitioner::FillRule;
use pathfinder_partitioner::mesh_pack::MeshPack;
use pathfinder_partitioner::partitioner::Partitioner;
use pathfinder_path_utils::cubic_to_quadratic::CubicToQuadraticTransformer;
use pathfinder_path_utils::stroke::{StrokeStyle, StrokeToFillIter};
use pathfinder_path_utils::transform::Transform2DPathIter;

use euclid::{Point2D, Transform2D};

use lyon_path::PathEvent;
use lyon_path::builder::{PathBuilder, FlatPathBuilder};

use app_units::Au;
use base64;
use fontsan;

use gfx_device_gl;
use gfx;
use gfx_glyph::{GlyphBrushBuilder, GlyphBrush};


type Resources = gfx_device_gl::Resources;
type Factory = gfx_device_gl::Factory;

static NEXT_FONT_KEY: AtomicUsize = ATOMIC_USIZE_INIT;

const CUBIC_TO_QUADRATIC_APPROX_TOLERANCE: f32 = 5.0;

const MESH_PACK_CACHE_SIZE: usize = 16;
lazy_static! {
    static ref MESH_PACK_CACHE: Mutex<LruCache<MeshPackCacheKey, Arc<Vec<u8>>>> = {
        Mutex::new(LruCache::new(MESH_PACK_CACHE_SIZE))
    };
}

static BUILTIN_FONTS: [(&'static str, &'static str); 1] = [
    ("Inconsolata", "test_data/Inconsolata-Regular.ttf"),
];

#[derive(Clone)]
struct PathDescriptor {
    path_index: usize,
    fill_rule: FillRule,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MeshPackCacheKey {
    builtin_font_name: String,
    glyph_ids: Vec<u32>,
}

#[derive(Clone, Copy, Debug)]
struct PartitionGlyph {
    id: u32,
    transform: Transform2D<f32>,
}

#[derive(Debug, Clone)]
pub struct FontRequest {
    pub path: String,
    font_index: u32,
    glyphs: Vec<PartitionGlyph>,
    point_size: f64
}

impl FontRequest {
    pub fn new<I: Into<String>>(path: I) -> FontRequest {
        FontRequest {
            path: path.into(),
            font_index: 0,
            glyphs: vec![ PartitionGlyph { id: 65, transform: Transform2D::identity() }],
            point_size: 10.0
        }
    }
}

impl PartialEq for FontRequest {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for FontRequest {
    
}

impl Hash for FontRequest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

pub struct ResourceManager {
    pub factory: gfx_device_gl::Factory,
    pub target: gfx::handle::RenderTargetView<Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    pub depth_stencil: gfx::handle::DepthStencilView<Resources, (gfx::format::D24_S8, gfx::format::Unorm)>,
    pub encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>
}

#[derive(Debug)]
pub enum FontError {
    FontContextCreationFailed,
    FontLoadingFailed,
    UnknownBuiltinFont,
    Base64DecodingFailed,
    FontSanitizationFailed
}

#[derive(Clone)]
enum FontRequestFace {
    /// One of the builtin fonts in `BUILTIN_FONTS`.
    Builtin(String),
    /// Base64-encoded OTF data.
    Custom(String),
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct FontKey(usize);

impl FontKey {
    fn new() -> FontKey {
        FontKey(NEXT_FONT_KEY.fetch_add(1, Ordering::SeqCst))
    }
}

struct PathPartitioningResult {
    encoded_data: Arc<Vec<u8>>,
    time: Duration,
}

impl PathPartitioningResult {
    fn compute(pack: &mut MeshPack,
               path_descriptors: &[PathDescriptor],
               paths: &[Vec<PathEvent>],
               approx_tolerance: Option<f32>)
               -> PathPartitioningResult {
        let timestamp_before = Instant::now();

        for (path, path_descriptor) in paths.iter().zip(path_descriptors.iter()) {
            let mut partitioner = Partitioner::new();
            if let Some(tolerance) = approx_tolerance {
                partitioner.builder_mut().set_approx_tolerance(tolerance);
            }

            path.iter().for_each(|event| partitioner.builder_mut().path_event(*event));
            partitioner.partition(path_descriptor.fill_rule);
            partitioner.builder_mut().build_and_reset();

            partitioner.mesh_mut().push_stencil_segments(
                CubicToQuadraticTransformer::new(path.iter().cloned(),
                                                 CUBIC_TO_QUADRATIC_APPROX_TOLERANCE));
            partitioner.mesh_mut().push_stencil_normals(
                CubicToQuadraticTransformer::new(path.iter().cloned(),
                                                 CUBIC_TO_QUADRATIC_APPROX_TOLERANCE));
            pack.push(partitioner.into_mesh());
        }

        let time_elapsed = timestamp_before.elapsed();

        let mut data_buffer = Cursor::new(vec![]);
        drop(pack.serialize_into(&mut data_buffer));

        PathPartitioningResult {
            encoded_data: Arc::new(data_buffer.into_inner()),
            time: time_elapsed,
        }
    }

    fn elapsed_ms(&self) -> f64 {
        self.time.as_secs() as f64 * 1000.0 + self.time.subsec_nanos() as f64 * 1e-6
    }
}

impl<'a> ResourceManager {
    pub fn new(factory: gfx_device_gl::Factory, target: gfx::handle::RenderTargetView<Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>, depth_stencil: gfx::handle::DepthStencilView<Resources, (gfx::format::D24_S8, gfx::format::Unorm)>, encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>) -> Self {
        ResourceManager {
            factory: factory,
            target: target,
            depth_stencil: depth_stencil,
            encoder: encoder
        }
    }

    pub fn get_font(&self, font_request: FontRequest) -> Result<Arc<Vec<u8>>, FontError> {
        // Check the cache for partitioned fonts
        let cache_key = MeshPackCacheKey {
            builtin_font_name: font_request.path.clone(),
            glyph_ids: font_request.glyphs.iter().map(|glyph| glyph.id).collect(),
        };


        if let Ok(mut mesh_library_cache) = MESH_PACK_CACHE.lock() {
            if let Some(cache_entry) = mesh_library_cache.get_mut(&cache_key) {
                return Ok((*cache_entry).clone())
            }
        }

        // Parse glyph data.
        let mut font_context = match FontContext::new() {
            Ok(font_context) => font_context,
            Err(_) => {
                println!("Failed to create a font context!");
                return Err(FontError::FontContextCreationFailed)
            }
        };

        let font_key = FontKey::new();
        let otf_data = try!(read_otf_data(&FontRequestFace::Builtin(font_request.path)));
        if font_context.add_font_from_memory(&font_key, otf_data, font_request.font_index).is_err() {
            return Err(FontError::FontLoadingFailed)
        }

        let font_instance = FontInstance {
            font_key: font_key,
            size: Au::from_f64_px(font_request.point_size),
        };

        // Read glyph info.
        let mut paths: Vec<Vec<PathEvent>> = vec![];
        let mut path_descriptors = vec![];

        for (glyph_index, glyph) in font_request.glyphs.iter().enumerate() {
            let glyph_key = GlyphKey::new(glyph.id, SubpixelOffset(0));

            // This might fail; if so, just leave it blank.
            match font_context.glyph_outline(&font_instance, &glyph_key) {
                Ok(glyph_outline) => {
                    paths.push(Transform2DPathIter::new(glyph_outline.iter(),
                                                        &glyph.transform).collect())
                }
                Err(_) => paths.push(vec![]),
            };

            path_descriptors.push(PathDescriptor {
                path_index: glyph_index,
                fill_rule: FillRule::Winding,
            })
        }

        // Partition the decoded glyph outlines.
        let mut pack = MeshPack::new();
        let path_partitioning_result = PathPartitioningResult::compute(&mut pack,
                                                                    &path_descriptors,
                                                                    &paths,
                                                                    None);

        // Build the response.
        let elapsed_ms = path_partitioning_result.elapsed_ms();
        println!("elapsed time [ms]: {}", elapsed_ms);


        if let Ok(mut mesh_library_cache) = MESH_PACK_CACHE.lock() {
            mesh_library_cache.insert(cache_key, path_partitioning_result.encoded_data.clone());
        }

        Ok(path_partitioning_result.encoded_data)
    }
}

// Fetches the OTF data.
fn read_otf_data(face: &FontRequestFace) -> Result<Arc<Vec<u8>>, FontError> {
    match *face {
        FontRequestFace::Builtin(ref builtin_font_name) => {
            // Read in the builtin font.
            match BUILTIN_FONTS.iter().filter(|& &(name, _)| name == builtin_font_name).next() {
                Some(&(_, path)) => {
                    let mut data = vec![];
                    File::open(path).expect("Couldn't find builtin font!")
                                    .read_to_end(&mut data)
                                    .expect("Couldn't read builtin font!");
                    Ok(Arc::new(data))
                }
                None => return Err(FontError::UnknownBuiltinFont),
            }
        }
        FontRequestFace::Custom(ref encoded_data) => {
            // Decode Base64-encoded OTF data.
            let unsafe_otf_data = match base64::decode(encoded_data) {
                Ok(unsafe_otf_data) => unsafe_otf_data,
                Err(_) => return Err(FontError::Base64DecodingFailed),
            };

            // Sanitize.
            match fontsan::process(&unsafe_otf_data) {
                Ok(otf_data) => Ok(Arc::new(otf_data)),
                Err(_) => return Err(FontError::FontSanitizationFailed),
            }
        }
    }
}
