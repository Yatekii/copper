use drawing;
use drawables;
use schema_parser::geometry;
use schema_parser::component;
use schema_parser::component::{ geometry as component_geometry };
use schema_parser::schema_file::ComponentInstance;

pub struct DrawableComponent {
    drawables: Vec<Box<drawables::Drawable>>,
    pub instance: Option<ComponentInstance>
}

impl DrawableComponent {
    pub fn new(
        component: component::Component,
        instance: ComponentInstance
    ) -> DrawableComponent {
        // Generate all shapes for the component
        let drawables = component.get_graphic_elements().iter()
            .filter_map(|shape| ge_to_drawable(&shape, &instance))
            .collect::<Vec<_>>();

        // TODO: reenable text
        // Generate the text for the component
        // drawables.extend(
        //     component.fields.iter()
        //                          .filter(|field| field.visible)
        //                          .map(|shape| field_to_drawable(resource_manager.clone(), &shape))
        // );

        let bb = component.get_boundingbox();

        DrawableComponent {
            drawables: drawables,
            instance: Some(instance)
        }
    }

    pub fn draw(&self, buffers: &mut drawing::Buffers){
        for drawable in &self.drawables {
            drawable.draw(buffers);
        }
    }
}

pub fn ge_to_drawable(
    shape: &component_geometry::GraphicElement,
    instance: &ComponentInstance
) -> Option<Box<drawables::Drawable>> {
    match shape {
        &component_geometry::GraphicElement::Rectangle { ref start, ref end, filled, .. } => {
            let r = geometry::SchemaRect::from_points(&[
                geometry::SchemaPoint2D::new(start.x, start.y),
                geometry::SchemaPoint2D::new(end.x, end.y)
            ]).translate(&instance.position.to_vector());
            Some(Box::new(drawables::loaders::load_rectangle(drawing::Color::new(0.61, 0.05, 0.04, 1.0), &r, filled)))
        }
        &component_geometry::GraphicElement::Circle { ref center, radius, filled, .. } => {
            Some(Box::new(drawables::loaders::load_circle(
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &(center.clone() + instance.position.to_vector()),
                radius, filled
            )))
        },
        &component_geometry::GraphicElement::Pin { ref orientation, ref position, length, ref name, number, number_size, name_size, .. } => {
            Some(Box::new(drawables::loaders::load_pin(
                &(position.clone() + instance.position.to_vector()),
                length as f32, orientation, name.clone(), number, number_size, name_size
            )))
        },
        &component_geometry::GraphicElement::Polygon { ref points, filled, .. } => {
            Some(Box::new(drawables::loaders::load_polygon(
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &points.iter().map(|point| geometry::SchemaPoint2D::new(point.x, point.y) + instance.position.to_vector()).collect(),
                filled
            )))
        },
        // &component_geometry::GraphicElement::TextField { ref content, ref position, ref orientation, .. } => {
        //     Some(Box::new(drawables::loaders::load_text(resource_manager, &geometry::SchemaPoint2D::new(position.x, position.y), content, 30.0, orientation, component::Justify::Center, component::Justify::Center)))
        // }
        _ => None
    }
}

// pub fn field_to_drawable<'a>(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, field: &component::Field) -> Box<drawables::Drawable> {
//     Box::new(drawables::loaders::load_text(resource_manager, &geometry::SchemaPoint2D::new(field.position.x, field.position.y), &field.text, field.dimension as f32, &field.orientation, field.hjustify.clone(), field.vjustify.clone()))
// }