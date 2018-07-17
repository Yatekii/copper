use std::sync::{
    Arc,
    RwLock,
};

use epoxy;

use gfx;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::format::Formatted;
use gfx_core;
use gfx_core::memory::Typed;
use gfx_gl;
use gfx_device_gl;

use state::schema::*;
use drawing;

use state::event::{Listener, EventMessage};

pub use drawing::schema::{
    DrawableWire,
    DrawableComponent,
    DrawableComponentInstance,
};
use drawing::drawables;

use parsing::schema_file::ComponentInstance;
use geometry::schema_elements::WireSegment;

use manipulation::library::Library;

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

struct GfxMachinery {
    factory: gfx_device_gl::Factory,
    device: gfx_device_gl::Device,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    target: gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    msaatarget: gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    msaaview: gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    program: gfx::PipelineState<gfx_device_gl::Resources, drawing::pipe::Meta>,
    program_render: gfx::PipelineState<gfx_device_gl::Resources, drawing::pipe_render::Meta>,
}

impl GfxMachinery {
    pub fn new() -> GfxMachinery {

        // Create a new device with a getter for GL calls.
        // This can be done via libepoxy which is a layer above GL and simplifies the retrieval of the function handles
        let (device, mut factory) = gfx_device_gl::create(epoxy::get_proc_addr);

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

        // TODO: what happens on resize???
        /* Create actual MSAA enabled RT */
        let (_, view_msaa, target_msaa) = create_render_target_msaa(
            &mut factory,
            1000,
            1000,
            8
        ).unwrap();

        GfxMachinery {
            factory: factory,
            device: device,
            encoder: encoder,
            target: target,
            msaatarget: target_msaa,
            msaaview: view_msaa,
            program: program,
            program_render: program_render,
        }
    }
}

pub struct SchemaDrawer {
    schema: Arc<RwLock<Schema>>,
    view_state: Arc<RwLock<ViewState>>,
    library: Arc<RwLock<Library>>,

    wires: Vec<DrawableWire>,
    components: RwLock<Vec<DrawableComponentInstance>>,

    // GL requirements
    gfx_machinery: Option<GfxMachinery>,
}

impl SchemaDrawer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, library: Arc<RwLock<Library>>) -> SchemaDrawer {
        SchemaDrawer {
            schema: schema,
            view_state: view_state,
            library: library,
            wires: Vec::new(),
            components: RwLock::from(Vec::new()),

            gfx_machinery: None,
        }
    }
    /// Issues draw calls to render the entire schema
    pub fn fill_buffers(&self, buffers: &mut drawing::Buffers) {
        for drawable in self.components.read().unwrap().iter() {
            // Unwrap should be ok as there has to be an instance for every component in the schema

            drawable.draw(buffers);
        }

        for wire in &self.wires {
            wire.draw(buffers);
        }
    }

    fn draw(&mut self) {
        // Init GL machinery in the first draw as we can't catch the realize event
        if self.gfx_machinery.is_none() {
            self.gfx_machinery = Some(GfxMachinery::new());
        }

        self.draw_frame();
        self.finalize_frame();
    }

    fn draw_frame(&mut self) {
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
        //println!("IBO Len {}", buffers.ibo.len());

        let mut gm = self.gfx_machinery.as_mut().unwrap();

        let mut view_state = &self.view_state.write().unwrap();

        // view_state.selected_component_uuid.map(|v| {
        //     visual_helpers::draw_selection_indicator(&mut buffers, v);
        // });

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

    fn finalize_frame(&mut self) {
        let mut gm = self.gfx_machinery.as_mut().unwrap();
        gm.encoder.flush(&mut gm.device);
        // TODO: swap buffers
        gm.device.cleanup();
    }
}

impl SchemaActor for SchemaDrawer {
    fn component_added(&self, instance: &ComponentInstance) {
        let library = self.library.write().unwrap();
        let component = library.get_component(instance);
        let mut instance = instance.clone();

        instance.set_component(component.clone());
        let drawable_component = DrawableComponentInstance {
            instance: instance,
            drawable: DrawableComponent::new(self.components.read().unwrap().len() as u32, component.clone()),
        };
        self.components.write().unwrap().push(drawable_component);
    }

    fn component_updated(&self, instance: &ComponentInstance) {

    }

    fn wire_added(&mut self, instance: WireSegment) {
        let dw = DrawableWire::from_schema((self.components.read().unwrap().len() + self.wires.len()) as u32, &instance);
    }

    fn wire_updated(&mut self, instance: WireSegment) {
        
    }
}

impl Listener for SchemaDrawer {
    fn receive(&mut self, msg: &EventMessage) {
        match msg {
            EventMessage::AddComponent(component) => {
                self.component_added(component)
            },
            EventMessage::ChangeComponent(component) => self.component_updated(component),
            EventMessage::DrawSchema => self.draw(),
            EventMessage::ResizeDrawArea(w, h) => {
                let gm = self.gfx_machinery.as_mut();
                gm.map(|m| {
                    m.target = create_render_target(*w, *h);
                    let (_, view_msaa, target_msaa) = create_render_target_msaa(&mut m.factory, *w, *h, 8 ).unwrap();
                    m.msaaview = view_msaa;
                    m.msaatarget = target_msaa;
                });
            },
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