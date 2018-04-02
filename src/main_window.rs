use std;

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

use gfx_gl;

use gfx_device_gl;

use epoxy;

use relm::Widget;
use relm_attributes::widget;

use self::Msg::*;

use std::time::{SystemTime, UNIX_EPOCH};

/* Defines for gfx-rs/OGL pipeline */
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [1.0, 0.2, 0.3, 1.0];


pub struct Model {
    gfx_factory: Option<gfx_device_gl::Factory>,
    gfx_device: Option<gfx_device_gl::Device>,
    gfx_encoder: Option<gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer> >,
    gfx_pso: Option<gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>>,
    gfx_slice: Option<gfx::Slice<gfx_device_gl::Resources>>,
    gfx_data: Option<pipe::Data<gfx_device_gl::Resources>>,
    width: i32,
    height: i32,
    ms: u64,
    nanos: u64,
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
            gfx_pso: None,
            gfx_slice: None,
            gfx_data: None,
            height: 0,
            width: 0,
            ms: 0,
            nanos: 0,
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            Realize => println!("realize!"), // This will never be called because relm applies this handler after the event
            Unrealize => println!("unrealize!"),
            RenderGl(context) => self.render_gl(context),
            Resize(w,h) => {
                println!("RenderArea size - w: {}, h: {}", w, h);
                self.model.width = w;
                self.model.height = h;

                if let Some(data) = self.model.gfx_data.as_mut() {
                    // Get dimensions of the GlArea
                    let dim: gfx::texture::Dimensions = (self.model.width as u16, self.model.height as u16, 1, gfx::texture::AaMode::Single);
                    println!("Texture dimension: w={}, h={}", self.model.width as u16, self.model.height as u16);
                    // Create a new RenderTarget with the dimensions
                    let (main_color, _ds_view) = gfx_device_gl::create_main_targets_raw(dim, ColorFormat::get_format().0, DepthFormat::get_format().0);
                    // Apply the RT
                    data.out = Typed::new(main_color);
                }
            }
        }
    }

    fn setup_render_context(&mut self) {
        let (vs_code, fs_code) = (
            include_bytes!("shaders/triangle_150_core.glslv").to_vec(),
            include_bytes!("shaders/triangle_150_core.glslf").to_vec(),
        );

        // Create a new device with a getter for GL calls.
        // This can be done via libepoxy which is a layer above GL and simplifies the retrieval of the function handles
        let (mut device, mut factory) = gfx_device_gl::create(epoxy::get_proc_addr);
        self.model.gfx_device = Some(device);

        // We need to select the proper FrameBuffer, as the default FrameBuffer is used by GTK itself to render the GUI
        // It then exposes a second FB which holds the RTV
        println!("Set the correct buffer");
        use gfx_device_gl::FrameBuffer;
        let mut cmdbuf = factory.create_command_buffer();
        unsafe {
            let mut fbo: i32 = 0;
            std::mem::transmute::<_, extern "system" fn(gfx_gl::types::GLenum, *mut gfx_gl::types::GLint) -> ()>(
                epoxy::get_proc_addr("glGetIntegerv")
            )(gfx_gl::DRAW_FRAMEBUFFER_BINDING, &mut fbo);
            println!("FBO number: {}", fbo);
            cmdbuf.display_fb = fbo as FrameBuffer;
        }
        
        // Create a new GL pipeline
        self.model.gfx_encoder = Some(gfx::Encoder::from(cmdbuf));
        self.model.gfx_pso = Some(factory.create_pipeline_simple(&vs_code, &fs_code, pipe::new()).unwrap());

        // Create our triangle VBO and remember the slice
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        self.model.gfx_slice = Some(slice);

        // Get initial dimensions of the GlArea
        let dim: gfx::texture::Dimensions = (self.model.width as u16, self.model.height as u16, 1, gfx::texture::AaMode::Single);
        println!("Texture dimension: w={}, h={}", self.model.width as u16, self.model.height as u16);
        // Create a initial RenderTarget with the dimensions
        let (main_color, _ds_view) = gfx_device_gl::create_main_targets_raw(dim, ColorFormat::get_format().0, DepthFormat::get_format().0);

        // Create the pipeline data struct
        self.model.gfx_data = Some(pipe::Data { vbuf: vertex_buffer, out: Typed::new(main_color) });

        self.model.gfx_factory = Some(factory);
    }

    fn prepare_frame(&mut self, context: gdk::GLContext) {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let ms = since_the_epoch.as_secs() * 1000;
        let nanos = since_the_epoch.subsec_nanos() as u64;
        println!("Time since last frame: {},{}", (ms + nanos / 1_000_000), (self.model.ms + self.model.nanos / 1_000_000));
        self.model.ms = ms;
        self.model.nanos = nanos;
        println!("render!");

        // Make the GlContext received from GTK the current one
        use gdk::GLContextExt;
        context.make_current();
    }

    fn draw_frame(&mut self) {
        let data = self.model.gfx_data.as_ref().unwrap();
        let encoder = self.model.gfx_encoder.as_mut().unwrap();
        let device =  self.model.gfx_device.as_mut().unwrap();
        let pso =  self.model.gfx_pso.as_ref().unwrap();
        let slice =  self.model.gfx_slice.as_ref().unwrap();
        println!("Pre clear!");
        encoder.clear(&data.out, CLEAR_COLOR);
        println!("Pre draw!");
        encoder.draw(slice, pso, data);
        println!("Pre flush");
        encoder.flush(device);
        println!("Pre cleanup!");
        device.cleanup();
    }

    fn finalize_frame(&mut self) {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let end = since_the_epoch.as_secs() * 1_000_000 + since_the_epoch.subsec_nanos() as u64 / 1000;
        let start = self.model.ms * 1000 + self.model.nanos / 1000;
        println!("Frametime in us: {},{}", end, start);
        println!("Frametime in us: {}", end - start);
    }


    fn render_gl(&mut self, context: gdk::GLContext) {
        self.prepare_frame(context);

        // Init GL machinery in the first draw as we can't catch the realize event
        if self.model.gfx_factory.is_none() {
            self.setup_render_context();
        }

        self.draw_frame();
        self.finalize_frame();
    }

    view! {
        gtk::Window {

            can_focus: false,
            border_width: 1,
            property_default_width: 400,
            property_default_height: 600,
            realize => Realize,

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