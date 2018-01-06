use std::cell::RefCell;
use std::rc::Rc;


use euclid;
use gfx_device_gl;


use drawing;
use drawables;
use geometry;
use schema_parser::component;
use schema_parser::component::geometry as component_geometry;
use resource_manager;
use schema_parser::component::geometry::Point;
use schema_parser::schema_file::ComponentInstance;


type Resources = gfx_device_gl::Resources;


pub struct DrawableComponent {
    pub component: component::Component,
    drawables: Vec<Box<drawables::Drawable>>,
    pub bounding_box: (Point, Point),
    pub instance: Option<ComponentInstance>
}

impl DrawableComponent {
    pub fn new(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, component: component::Component) -> DrawableComponent {
        let mut drawables: Vec<Box<drawables::Drawable>> = component.graphic_elements.iter()
                                                        .filter_map(|shape| ge_to_drawable(resource_manager.clone(), &shape))
                                                        .collect::<Vec<_>>();
        drawables.extend(
            component.fields.iter()
                                 .filter(|field| field.visible)
                                 .map(|shape| field_to_drawable(resource_manager.clone(), &shape))
        );
        let bb = component.get_boundingbox();

        DrawableComponent {
            component: component,
            drawables: drawables,
            bounding_box: bb,
            instance: None
        }
    }

    pub fn draw(&self, resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, perspective: &geometry::TSchemaScreen){
        for drawable in &self.drawables {
            drawable.draw(resource_manager.clone(), perspective.clone());
        }
    }

    pub fn get_bounding_box(&self) -> &(Point, Point) {
        &self.bounding_box
    }
}

pub fn ge_to_drawable(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, shape: &component_geometry::GraphicElement) -> Option<Box<drawables::Drawable>> {
    match shape {
        &component_geometry::GraphicElement::Rectangle { ref start, ref end, filled, .. } => {
            let r = geometry::SchemaRect::from_points(
                &[geometry::SchemaPoint2D::new(start.x, start.y), geometry::SchemaPoint2D::new(end.x, end.y)]
            );
            Some(Box::new(drawables::loaders::load_rectangle(resource_manager, drawing::Color::new(0.61, 0.05, 0.04, 1.0), &r, filled)))
        }
        &component_geometry::GraphicElement::Circle { ref center, radius, filled, .. } => {
            let center = center.to_euclid();
            let center = geometry::SchemaPoint2D::new(center.x, center.y);
            Some(Box::new(drawables::loaders::load_circle(resource_manager, drawing::Color::new(0.61, 0.05, 0.04, 1.0), center, radius, filled)))
        },
        &component_geometry::GraphicElement::Pin { ref orientation, ref position, length, ref name, number, number_size, name_size, .. } => {
            let pos = position.to_euclid();
            let pos = geometry::SchemaPoint2D::new(pos.x, pos.y);
            Some(Box::new(drawables::loaders::load_pin(resource_manager, pos, length as f32, orientation, name.clone(), number, number_size, name_size)))
        },
        &component_geometry::GraphicElement::Polygon { ref points, filled, .. } => {
            Some(Box::new(drawables::loaders::load_polygon(resource_manager, drawing::Color::new(0.61, 0.05, 0.04, 1.0), points, filled)))
        },
        &component_geometry::GraphicElement::TextField { ref content, ref position, ref orientation, .. } => {
            Some(Box::new(drawables::loaders::load_text(resource_manager, position, content, 30.0, orientation, component::Justify::Center, component::Justify::Center)))
        }
        _ => None
    }
}

pub fn field_to_drawable<'a>(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, field: &component::Field) -> Box<drawables::Drawable> {
    Box::new(drawables::loaders::load_text(resource_manager, &field.position, &field.text, field.dimension as f32, &field.orientation, field.hjustify.clone(), field.vjustify.clone()))
}