use std::collections::{
    HashMap,
    VecDeque,
};

use std::panic;

use epoxy;
use uuid::Uuid;

use gfx;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::format::Formatted;
use gfx_core;
use gfx_core::memory::Typed;
use gfx_gl;
use gfx_device_gl;

use drawing;
use drawing::drawables;
use drawing::drawables::Drawable;

use state::schema::ViewState;

/* Defines for gfx-rs/OGL pipeline */

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const CLEAR_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];

const RENDER_CANVAS: [drawing::VertexRender; 6] = [
    drawing::VertexRender { position: [ -1.0, -1.0 ] },
    drawing::VertexRender { position: [  1.0, -1.0 ] },
    drawing::VertexRender { position: [  -1.0,  1.0 ] },
    drawing::VertexRender { position: [ 1.0, 1.0 ] },
    drawing::VertexRender { position: [  -1.0, 1.0 ] },
    drawing::VertexRender { position: [  1.0,  -1.0 ] }
];

/// This is a struct to hold the `GfxMachinery`.
/// It should only ever be used internally inside an Option<T> such that it can be initialized before it is ready.
struct InternalGfxMachinery {
    factory: gfx_device_gl::Factory,
    device: gfx_device_gl::Device,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    target: gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    msaatarget: gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    msaaview: gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    program: gfx::PipelineState<gfx_device_gl::Resources, drawing::pipe::Meta>,
    program_render: gfx::PipelineState<gfx_device_gl::Resources, drawing::pipe_render::Meta>,
}

impl InternalGfxMachinery {
    /// Creates a new GfxMachinery that is ready to draw.
    /// Returns None if no GfxContext is present.
    /// Warning: Only call after an OGL context has been created and made the current one.
    /// Warning: Will interfere with other drawing activities if the context is shared with other drawers.
    pub fn new() -> Option<Self> {
        // Create a new device with a getter for GL calls.
        // This can be done via libepoxy which is a layer above GL and simplifies the retrieval of the function handles
        let (device, mut factory) =
            if let Ok((d, f)) = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                gfx_device_gl::create(epoxy::get_proc_addr)
            })) {
                (d, f)
            } else {
                return None;
            };

        // Load the shader to draw the geometries
        let shader = factory.link_program(&drawables::loaders::VS_CODE, &drawables::loaders::FS_CODE).unwrap();
        // Make the shader produce an MSAA output
        let mut rasterizer = gfx::state::Rasterizer::new_fill();
        rasterizer.samples = Some(gfx::state::MultiSample);
        let program = factory.create_pipeline_from_program(
            &shader,
            gfx::Primitive::TriangleList,
            rasterizer,
            drawing::pipe::new()
        ).unwrap();

        // Load the shader to resolve the MSAA texture
        let shader = factory.link_program(&drawables::loaders::VS_RENDER_CODE, &drawables::loaders::FS_RENDER_CODE).unwrap();
        let program_render = factory.create_pipeline_from_program(
            &shader,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer::new_fill(),
            drawing::pipe_render::new()
        ).unwrap();

        // We need to select the proper FrameBuffer, as the default FrameBuffer is used by GTK itself to render the GUI
        // It then exposes a second FB which holds the RTV
        use gfx_device_gl::FrameBuffer;
        let mut cmdbuf = factory.create_command_buffer();
        unsafe {
            let mut fbo: i32 = 0;
            std::mem::transmute::<_, extern "system" fn(gfx_gl::types::GLenum, *mut gfx_gl::types::GLint) -> ()>(
                epoxy::get_proc_addr("glGetIntegerv")
            )(gfx_gl::DRAW_FRAMEBUFFER_BINDING, &mut fbo);
            cmdbuf.display_fb = fbo as FrameBuffer;
        }

        // Create a new GL pipeline with the just aquired draw framebuffer
        let encoder = gfx::Encoder::from(cmdbuf);

        let target = create_render_target(0, 0);

        /* Create actual MSAA enabled RT */
        let (_, view_msaa, target_msaa) = create_render_target_msaa(
            &mut factory,
            1000,
            1000,
            8
        ).unwrap();

        Some(Self {
            factory: factory,
            device: device,
            encoder: encoder,
            target: target,
            msaatarget: target_msaa,
            msaaview: view_msaa,
            program: program,
            program_render: program_render,
        })
    }
}

enum GfxTask {
    Resize(u16, u16)
}

/// A basic drawing util, which simply draws to the current OGL context.
/// Warning: `GfxMachinery` does NOT create an OGL context itself, but can be instantiated before the context exists.
pub struct GfxMachinery {
    drawables: Vec<(Uuid, Box<dyn Drawable>)>,
    drawable_map: HashMap<Uuid, usize>,

    machinery: Option<InternalGfxMachinery>,
    gfx_tasks: VecDeque<GfxTask>,
}

impl GfxMachinery {
    /// Creates a new `GfxMachinery`.
    /// Can be used before the OGL context exists.
    pub fn new() -> Self {
        GfxMachinery {
            drawables: Vec::new(),
            drawable_map: HashMap::new(),
            machinery: None,
            gfx_tasks: VecDeque::new(),
        }
    }

    /// Actually fills the buffers and starts the program.
    /// Warning: Will panic if called before the OGL context exists.
    fn draw_frame(&mut self, view_state: &ViewState) {
        // Create empty buffers
        let vbo = Vec::<drawing::Vertex>::new();
        let ibo = Vec::<u32>::new();
        let abo = Vec::<drawing::Attributes>::new();
        let mut buffers = drawing::Buffers {
            vbo: vbo,
            ibo: ibo,
            abo: abo,
        };

        // Fill buffers
        self.fill_buffers(&mut buffers);

        let gm = self.machinery.as_mut().unwrap();

        // Create VBO
        let (vbo, ibo) = gm.factory.create_vertex_buffer_with_slice(
            &buffers.vbo[..],
            &buffers.ibo[..]
        );

        // Create per drawable attributes buffer
        let attributes = gm.factory.create_constant_buffer(800);

        // Create bundle
        let buf = gm.factory.create_constant_buffer(1);
        let bundle = gfx::pso::bundle::Bundle::new(
            ibo,
            gm.program.clone(),
            drawing::pipe::Data {
                vbuf: vbo,
                globals: buf,
                out: gm.msaatarget.clone(),
                attributes: attributes,
            }
        );
        let perspective = view_state.current_perspective.clone();
        let globals = drawing::Globals {
            perspective: perspective.into()
        };

        // Clear the canvas
        gm.encoder.clear(&mut gm.msaatarget, CLEAR_COLOR);

        // Add bundle to the pipeline
        gm.encoder.update_constant_buffer(&bundle.data.globals, &globals);
        gm.encoder.update_buffer(&bundle.data.attributes, &buffers.abo, 0).unwrap();
        bundle.encode(&mut gm.encoder);

        // TODO: Put to another location as this never changes and doesn't need to be done each frame
        let (vertex_buffer, slice) = gm.factory.create_vertex_buffer_with_slice(&RENDER_CANVAS, ());

        // TODO: Put to another location as this never changes and doesn't need to be done each frame
        use gfx::Factory;
        let sampler = gm.factory.create_sampler(gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Trilinear,
            gfx::texture::WrapMode::Tile,
        ));

        // Finalize image with render to final target
        let bundle = gfx::pso::bundle::Bundle::new(
            slice,
            gm.program_render.clone(),
            drawing::pipe_render::Data {
                vbuf: vertex_buffer,
                texture: (gm.msaaview.clone(), sampler),
                out: gm.target.clone()
            }
        );

        bundle.encode(&mut gm.encoder);
    }

    /// Flushes the pipeline and does the cleanup.
    /// TODO: Swapping buffers is not done yet.
    /// Warning: Will panic if the OGL context does not exist.
    fn finalize_frame(&mut self) {
        let gm = self.machinery.as_mut().unwrap();
        gm.encoder.flush(&mut gm.device);
        // TODO: swap buffers
        gm.device.cleanup();
    }

    /// Fills the Vertex buffers with `Drawables`.
    /// This is safe to be called any time.
    fn fill_buffers(&self, buffers: &mut drawing::Buffers) {
        for drawable in self.drawables.iter() {
            drawable.1.draw(buffers);
        }
    }

    /// Call to render a new frame with the loaded buffers.
    /// Call this when the OGL context to be drawn on is ready, e.g. in the `render` callback of a `GdkGLArea`.
    /// Warning: Will result in panic if called before the OGL context is active.
    pub fn draw(&mut self, view_state: &ViewState) {
        // Init GL machinery in the first draw as we can't catch the realize event
        if self.machinery.is_none() {
            self.machinery = InternalGfxMachinery::new();
        }
        if self.machinery.is_some() {
            let tasks: Vec<GfxTask> = self.gfx_tasks.drain(..).collect();
            tasks.iter().for_each(|t| match t {
                GfxTask::Resize(w, h) => {
                    println!("Resizing");
                    self.resize_target(*w, *h)
                }
            });
            self.draw_frame(view_state);
            self.finalize_frame();
        }
    }

    /// Adds a [`Drawable`] to be drawn in the next frame.
    pub fn add_drawable(&mut self, uuid: &Uuid, mut drawable: Box<dyn Drawable>) {
        let drawable_id = self.drawables.len();
        drawable.set_id(drawable_id as u32);
        self.drawables.push((uuid.clone(), drawable));
        self.drawable_map.insert(uuid.clone(), drawable_id);
    }

    /// Removes the Drawable matching the given Uuid from the drawables
    pub fn remove_drawable(&mut self, uuid: &Uuid) {
        let to_remove_id = self.drawable_map.get(uuid).map(|d| *d);
        if let Some(drawable_id) = to_remove_id {
            if self.drawables.len() > 1 {
                self.drawables.swap_remove(drawable_id);
                self.drawables[drawable_id].1.set_id(drawable_id as u32);
                self.drawable_map.remove(uuid);
                self.drawable_map.insert(self.drawables[drawable_id].0.clone(), drawable_id);
            } else {
                self.drawables.remove(drawable_id);
            }
        }
    }

    /// Clears the held drawables
    pub fn clear_drawables(&mut self) {
        self.drawables.clear();
        self.drawable_map.clear();
    }

    /// Resizes the render target.
    pub fn resize_target(&mut self, w: u16, h: u16) {
        let gm = self.machinery.as_mut();
        let requires_task = gm.map_or_else(
            || false,
            |m| {
                m.target = create_render_target(w, h);
                let (_, view_msaa, target_msaa) = create_render_target_msaa(&mut m.factory, w, h, 8 ).unwrap();
                m.msaaview = view_msaa;
                m.msaatarget = target_msaa;
                true
            }
        );
        if !requires_task {
            self.gfx_tasks.push_back(GfxTask::Resize(w, h));
        }
    }

    /// Returns any drawable with the `uuid` assigned.
    /// Safe to be called any time.
    pub fn get_drawable_mut(&mut self, uuid: &Uuid) -> Option<&mut dyn Drawable> {
        let id = self.drawable_map.get(uuid).map(|i| *i);
        if let Some(id) = id {
            Some(&mut *self.drawables[id].1)
        } else {
            None
        }
    }
}

/* GFX HELPERS */

/// Creates a new render target with msaa enabled. This should be used if the MSAA resolve pass has to be done by hand.
fn create_render_target_msaa<T: gfx_core::format::RenderFormat + gfx_core::format::TextureFormat, R: gfx_core::Resources, F> (
    factory: &mut F,
    width: gfx_core::texture::Size,
    height: gfx_core::texture::Size,
    msaa: u8
) -> Result<
    (
        gfx_core::handle::Texture<R, T::Surface>,
        gfx_core::handle::ShaderResourceView<R, T::View>,
        gfx_core::handle::RenderTargetView<R, T>
    ),
    gfx_core::factory::CombinedError
> where F: gfx_core::factory::Factory<R> {
    let kind = gfx_core::texture::Kind::D2(width, height, gfx_core::texture::AaMode::Multi(msaa));
    let levels = 1;
    let cty = <T::Channel as gfx_core::format::ChannelTyped>::get_channel_type();
    let tex = try!(factory.create_texture(kind, levels, gfx_core::memory::Bind::RENDER_TARGET | gfx_core::memory::Bind::SHADER_RESOURCE, gfx_core::memory::Usage::Data, Some(cty)));
    let view = try!(factory.view_texture_as_shader_resource::<T>(&tex, (levels, levels), gfx::format::Swizzle::new()));
    let target = try!(factory.view_texture_as_render_target(&tex, 0, None));
    Ok((tex, view, target))
}

/// Creates a new render target with given size. Used to draw the final resolved render onto.
fn create_render_target(w: u16, h: u16) -> gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)> {
    // Get initial dimensions of the GlArea
    let dim: gfx::texture::Dimensions = (
        w,
        h,
        1,
        gfx::texture::AaMode::Single
    );

    // Create a initial RenderTarget with the dimensions
    let (target, _ds_view) = gfx_device_gl::create_main_targets_raw(dim, ColorFormat::get_format().0, DepthFormat::get_format().0);
    // Create the pipeline data struct
    Typed::new(target)
}