use std;
use std::time::{SystemTime, UNIX_EPOCH};
use env;

use gtk;
use gtk::{
    ButtonExt,
    ContainerExt,
    Inhibit,
    OrientableExt,
    WidgetExt,
    BoxExt,
    GtkWindowExt,
    GLAreaExt,
    Orientation::Vertical,
};

use gdk;

use gfx;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::format::Formatted;

use gfx_core::memory::Typed;
use gfx_core;

use gfx_gl;

use gfx_device_gl;

use epoxy;

use relm::Widget;
use relm_attributes::widget;

use self::Msg::*;

use drawing;
use drawables;

use schema;
use library;

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

pub struct Model {
    gfx_factory: Option<gfx_device_gl::Factory>,
    gfx_device: Option<gfx_device_gl::Device>,
    gfx_encoder: Option<gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer> >,
    gfx_target: Option<gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>>,
    gfx_msaatarget: Option<gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>>,
    gfx_msaaview: Option<gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>>,
    program: Option<gfx::PipelineState<gfx_device_gl::Resources, drawing::pipe::Meta>>,
    program_render: Option<gfx::PipelineState<gfx_device_gl::Resources, drawing::pipe_render::Meta>>,
    width: i32,
    height: i32,
    ms: u64,
    nanos: u64,
    view_state: drawing::ViewState,
    schema: schema::Schema,
    title: String
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    Realize,
    Unrealize,
    RenderGl(gdk::GLContext),
    Resize(i32, i32),
}

#[widget]
impl Widget for Win {
    // The initial model.
    fn model() -> Model {
        Model {
            gfx_factory: None,
            gfx_device: None,
            gfx_encoder: None,
            gfx_target: None,
            gfx_msaatarget: None,
            gfx_msaaview: None,
            program: None,
            program_render: None,
            height: 0,
            width: 0,
            ms: 0,
            nanos: 0,
            view_state: drawing::ViewState::new(1, 1),
            schema: schema::Schema::new(),
            title: "Schema Renderer".to_string(),
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        //println!("{:?}", event);
        match event {
            Quit => gtk::main_quit(),
            Realize => println!("realize!"), // This will never be called because relm applies this handler after the event
            Unrealize => println!("unrealize!"),
            RenderGl(context) => self.render_gl(context),
            Resize(w,h) => {
                println!("RenderArea size - w: {}, h: {}", w, h);
                self.model.width = w;
                self.model.height = h;
                self.model.view_state.update_from_resize(w as u32, h as u32);

                // Get initial dimensions of the GlArea
                let dim: gfx::texture::Dimensions = (
                    self.model.width as u16,
                    self.model.height as u16,
                    1,
                    gfx::texture::AaMode::Single
                );
                
                // Create a initial RenderTarget with the dimensions
                let (target, _ds_view) = gfx_device_gl::create_main_targets_raw(dim, ColorFormat::get_format().0, DepthFormat::get_format().0);
                // Create the pipeline data struct
                self.model.gfx_target = Some(Typed::new(target));
            }
        }
    }

    fn load_schema(&mut self) {
        /*
        * L O A D   S C H E M A
        */

        // Load library and schema file
        let args: Vec<String> = env::args().collect();
        if args.len() != 3 {
            println!("Please specify a .lib and a .sch file.");
            ::std::process::exit(1);
        }

        // Create a new Library from a file specified on the commandline
        let library = library::Library::new(&args[1]).unwrap();

        // Load a schema form a file specified on the commandline
        self.model.schema.load(&library, args[2].clone());

        // Zoom to BB
        let bb = self.model.schema.get_bounding_box();
        self.model.view_state.update_from_box_pan(bb);
    }

    fn setup_render_context(&mut self) {

        // Create a new device with a getter for GL calls.
        // This can be done via libepoxy which is a layer above GL and simplifies the retrieval of the function handles
        let (device, mut factory) = gfx_device_gl::create(epoxy::get_proc_addr);
        self.model.gfx_device = Some(device);

        // Create the program
        let shader = factory.link_program(&drawables::loaders::VS_CODE, &drawables::loaders::FS_CODE).unwrap();
        let mut rasterizer = gfx::state::Rasterizer::new_fill();
        rasterizer.samples = Some(gfx::state::MultiSample);
        self.model.program = Some(factory.create_pipeline_from_program(
            &shader,
            gfx::Primitive::TriangleList,
            rasterizer,
            drawing::pipe::new()
        ).unwrap());

        let shader = factory.link_program(&drawables::loaders::VS_RENDER_CODE, &drawables::loaders::FS_RENDER_CODE).unwrap();
        let rasterizer = gfx::state::Rasterizer::new_fill();
        self.model.program_render = Some(factory.create_pipeline_from_program(
            &shader,
            gfx::Primitive::TriangleList,
            rasterizer,
            drawing::pipe_render::new()
        ).unwrap());

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
        
        // Create a new GL pipeline
        self.model.gfx_encoder = Some(gfx::Encoder::from(cmdbuf));

        // Get initial dimensions of the GlArea
        let dim: gfx::texture::Dimensions = (
            self.model.width as u16,
            self.model.height as u16,
            1,
            gfx::texture::AaMode::Single
        );
        
        // Create a initial RenderTarget with the dimensions
        let (target, _ds_view) = gfx_device_gl::create_main_targets_raw(dim, ColorFormat::get_format().0, DepthFormat::get_format().0);
        // Create the pipeline data struct
        self.model.gfx_target = Some(Typed::new(target));

        /* Create actual MSAA enabled RT */
        let (_, view_msaa, target_msaa) = create_render_target_msaa(
            &mut factory,
            self.model.width as u16,
            self.model.height as u16,
            8
        ).unwrap();

        self.model.gfx_msaatarget = Some(target_msaa);
        self.model.gfx_msaaview = Some(view_msaa);

        self.model.gfx_factory = Some(factory);
    }

    fn prepare_frame(&mut self, context: gdk::GLContext) {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let ms = since_the_epoch.as_secs() * 1000;
        let nanos = since_the_epoch.subsec_nanos() as u64;
        // println!("Time since last frame: {},{}", (ms + nanos / 1_000_000), (self.model.ms + self.model.nanos / 1_000_000));
        self.model.ms = ms;
        self.model.nanos = nanos;

        // Make the GlContext received from GTK the current one
        use gdk::GLContextExt;
        context.make_current();
    }

    fn draw_frame(&mut self) {
        let encoder = self.model.gfx_encoder.as_mut().unwrap();
        let device = self.model.gfx_device.as_mut().unwrap();
        let target = self.model.gfx_target.as_mut().unwrap();
        let target_msaa = self.model.gfx_msaatarget.as_mut().unwrap();
        let view_msaa = self.model.gfx_msaaview.as_mut().unwrap();
        let factory = self.model.gfx_factory.as_mut().unwrap();
        let program = self.model.program.as_mut().unwrap();
        let program_render = self.model.program_render.as_mut().unwrap();

        // Clear the canvas
        encoder.clear(target_msaa, CLEAR_COLOR);

        // Create empty buffers
        let vbo = Vec::<drawing::Vertex>::new();
        let ibo = Vec::<u32>::new();
        let mut buffers = drawing::Buffers {
            vbo: vbo,
            ibo: ibo
        };

        // Fill buffers
        self.model.schema.draw(&mut buffers);

        let (vbo, ibo) = factory.create_vertex_buffer_with_slice(
            &buffers.vbo[..],
            &buffers.ibo[..]
        );

        // Create bundle
        let buf = factory.create_constant_buffer(1);
        let bundle = gfx::pso::bundle::Bundle::new(
            ibo,
            program.clone(),
            drawing::pipe::Data {
                vbuf: vbo,
                locals: buf,
                out: target_msaa.clone()
            }
        );
        let locals = drawing::Locals {
            perspective: self.model.view_state.current_perspective.to_row_arrays()
        };

        // Add bundle to the pipeline
        encoder.update_constant_buffer(&bundle.data.locals, &locals);
        bundle.encode(encoder);

        // TODO: Put to another location as this never changes and doesn't need to be done each frame
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&RENDER_CANVAS, ());

        // TODO: Put to another location as this never changes and doesn't need to be done each frame
        use gfx::Factory;
        let sampler = factory.create_sampler(gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Trilinear,
            gfx::texture::WrapMode::Tile,
        ));

        // Finalize image with render to final target
        let bundle = gfx::pso::bundle::Bundle::new(
            slice,
            program_render.clone(),
            drawing::pipe_render::Data {
                vbuf: vertex_buffer,
                texture: (view_msaa.clone(), sampler), 
                out: target.clone()
            }
        );

        bundle.encode(encoder);

        // TODO: draw visual helpers again
        // Draw the coords and the kicad space coords at the cursor
        // let cp = view_state.cursor.clone();
        // let mut c = view_state.cursor.clone();
        // c.x =  (c.x / view_state.width  as f32) * 2.0 - 1.0;
        // c.y = -(c.y / view_state.height as f32) * 2.0 + 1.0;
        // let kc = view_state.current_perspective.inverse().unwrap().transform_point3d(&c.to_3d());
        // visual_helpers::draw_coords_at_cursor(resource_manager.clone(), cp.x, cp.y, c.x, c.y, kc.x, kc.y);
    }

    fn finalize_frame(&mut self) {
        let encoder = self.model.gfx_encoder.as_mut().unwrap();
        let device = self.model.gfx_device.as_mut().unwrap();
        encoder.flush(device);
        // TODO: swap buffers
        device.cleanup();
        // let start = SystemTime::now();
        // let since_the_epoch = start.duration_since(UNIX_EPOCH)
        //     .expect("Time went backwards");
        // let end = since_the_epoch.as_secs() * 1_000_000 + since_the_epoch.subsec_nanos() as u64 / 1000;
        // let start = self.model.ms * 1000 + self.model.nanos / 1000;
        // println!("Frametime in us: {}", end - start);
    }


    fn render_gl(&mut self, context: gdk::GLContext) {
        self.prepare_frame(context);

        // Init GL machinery in the first draw as we can't catch the realize event
        if self.model.gfx_factory.is_none() {
            self.load_schema();
            self.setup_render_context();
        }

        self.draw_frame();
        self.finalize_frame();
    }

    view! {
        gtk::Window {
            can_focus: false,
            border_width: 1,
            property_default_width: 1800,
            property_default_height: 1000,
            realize => Realize,
            title: &self.model.title,

            child: {
                expand: true,
                fill: true,
            },

            gtk::Box {
                orientation: Vertical,
                can_focus: false,
                spacing: 6,
                realize => Realize,


                gtk::GLArea {
                    can_focus: false,
                    hexpand: true,
                    vexpand: true,
                    realize => Realize,
                    unrealize => Unrealize,
                    resize(_area, width, height) => Resize(width, height),
                    render(area, context) => ({
                        let rgl = RenderGl(context.clone());
                        area.queue_render();
                        rgl
                    }, Inhibit(true)),
                },

                gtk::Button {
                    label: "KEK",
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}


fn create_render_target_msaa<T: gfx_core::format::RenderFormat + gfx_core::format::TextureFormat, R: gfx_core::Resources, F>
                           (factory: &mut F, width: gfx_core::texture::Size, height: gfx_core::texture::Size, msaa: u8)
                            -> Result<(gfx_core::handle::Texture<R, T::Surface>, gfx_core::handle::ShaderResourceView<R, T::View>, gfx_core::handle::RenderTargetView<R, T>), gfx_core::factory::CombinedError>
                            where F: gfx_core::factory::Factory<R>
    {
        let kind = gfx_core::texture::Kind::D2(width, height, gfx_core::texture::AaMode::Multi(msaa));
        let levels = 1;
        let cty = <T::Channel as gfx_core::format::ChannelTyped>::get_channel_type();
        let tex = try!(factory.create_texture(kind, levels, gfx_core::memory::Bind::RENDER_TARGET | gfx_core::memory::Bind::SHADER_RESOURCE, gfx_core::memory::Usage::Data, Some(cty)));
        let view = try!(factory.view_texture_as_shader_resource::<T>(&tex, (levels, levels), gfx::format::Swizzle::new()));
        let target = try!(factory.view_texture_as_render_target(&tex, 0, None));
        Ok((tex, view, target))
    }