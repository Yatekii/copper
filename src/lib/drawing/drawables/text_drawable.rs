use drawing;
use geometry;
use geometry::*;
use parsing::kicad::component_library::*;


pub struct TextDrawable {
    pub position: geometry::Point2D,
    pub content: String,
    pub dimension: f32,
    pub orientation: TextOrientation,
    pub hjustify: Justify,
    pub vjustify: Justify
}

impl super::Drawable for TextDrawable {
    fn draw(&self, _buffers: &mut drawing::Buffers) {
        // let (w, h, _z, _aamode) = resource_manager.borrow().target.clone().get_dimensions();

        // // Transform Schema coords to Screen coords
        // let position_schema = geometry::euclid::TypedPoint3D<f32, SchemaSpace>::new(self.position.x as f32, self.position.y as f32, 0.0);
        // let mut position_screen = perspective.transform_point3d(&position_schema);
        // position_screen.x = (position_screen.x + 1.0) / 2.0 *  (w as f32);
        // position_screen.y = (position_screen.y - 1.0) / 2.0 * -(h as f32);

        // let px_per_schema = match self.orientation {
        //     component_geometry::TextOrientation::Horizontal => (w as f32) / (2.0 / perspective.m11),
        //     component_geometry::TextOrientation::Vertical => (h as f32) / (2.0 / perspective.m22)
        // };

        // let font = {
        //     let rm = resource_manager.borrow_mut();
        //     rm.get_font(resource_manager::FontKey::new("test_data/Inconsolata-Regular.ttf"))
        // };

        // let mut layout = gfx_glyph::Layout::default();

        // match self.hjustify {
        //     component::Justify::Left => { layout = layout.h_align(gfx_glyph::HorizontalAlign::Left); },
        //     component::Justify::Right => { layout = layout.h_align(gfx_glyph::HorizontalAlign::Right); },
        //     component::Justify::Center => { layout = layout.h_align(gfx_glyph::HorizontalAlign::Center); },
        //     _ => {}
        // }

        // // TODO: Add Center & Bottom (needs pull request to gfx_glyph)
        // match self.vjustify {
        //     component::Justify::Top => { layout = layout.v_align(gfx_glyph::VerticalAlign::Top); },
        //     component::Justify::Bottom => { layout = layout.v_align(gfx_glyph::VerticalAlign::Top); },
        //     component::Justify::Center => { layout = layout.v_align(gfx_glyph::VerticalAlign::Top); },
        //     _ => {}
        // }

        // // let transform = {
        // //     let aspect = h as f32 / w as f32;
        // //     let zoom = 1.0;
        // //     let origin = (0.0, 0.0); // top-corner: `let origin = (1.0 * aspect, -1.0);`
        // //     let projection = euclid::TypedTransform3D::<f32, SchemaSpace, SchemaSpace>::ortho(
        // //         origin.0 - zoom * aspect,
        // //         origin.0 + zoom * aspect,
        // //         origin.1 - zoom,
        // //         origin.1 + zoom,
        // //         1.0,
        // //         -1.0,
        // //     );
        // //     let mut m = euclid::TypedTransform3D::<f32, SchemaSpace, SchemaSpace>::row_major(
        // //         0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
        // //     );
        // //     projection.post_mul(&m).post_mul(&projection.inverse().unwrap())
        // // };

        // let transform = geometry::TSchemaSchema::identity();

        // let section = gfx_glyph::Section {
        //     text: &self.content,
        //     screen_position: (
        //         position_screen.x as f32,
        //         position_screen.y as f32
        //     ),
        //     scale: gfx_glyph::Scale::uniform(self.dimension * px_per_schema),
        //     layout: layout,
        //     ..gfx_glyph::Section::default()
        // };

        // let mut f = font.borrow_mut();
        // f.queue(section);
        // let t = resource_manager.borrow().target.clone();
        // let r = resource_manager.borrow().depth_stencil.clone();
        // f.draw_queued_with_transform(transform.to_row_arrays(), &mut resource_manager.borrow_mut().encoder, &t, &r).unwrap();
    }
    fn get_transform(&self) -> Matrix4 { Matrix4::identity() }
    fn set_transform(&mut self, _transform: &Matrix4) {}
    fn set_id(&mut self, _id: u32) {}
}