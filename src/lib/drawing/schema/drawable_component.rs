use drawing;
use drawing::drawables;
use geometry::{ Point2D, AABB };
use geometry::schema_elements::*;
use parsing::component;
use parsing::schema_file::ComponentInstance;
use std::rc::Weak;

pub struct DrawableComponent {
    drawables: Vec<Box<drawables::Drawable>>
}

impl DrawableComponent {
    pub fn new(
        component: Weak<component::Component>,
        instance: ComponentInstance
    ) -> DrawableComponent {
        // Generate all shapes for the component
        let ocomponent = component.upgrade();
        let drawables = ocomponent.map(|c| c.get_graphic_elements().iter()
            .filter_map(|shape| ge_to_drawable(&shape, &instance))
            .collect::<Vec<_>>()
        );

        // TODO: reenable text
        // Generate the text for the component
        // drawables.extend(
        //     component.fields.iter()
        //                          .filter(|field| field.visible)
        //                          .map(|shape| field_to_drawable(resource_manager.clone(), &shape))
        // );

        DrawableComponent {
            drawables: drawables.unwrap_or(Vec::new())
        }
    }

    pub fn draw(&self, buffers: &mut drawing::Buffers){
        for drawable in &self.drawables {
            drawable.draw(buffers);
        }
    }
}

pub fn ge_to_drawable(
    shape: &GraphicElement,
    instance: &ComponentInstance
) -> Option<Box<drawables::Drawable>> {
    match shape {
        &GraphicElement::Rectangle { start, end, filled, .. } => {
            use utils::traits::Translatable;
            let mins = Point2D::new(
                if start.x > end.x { end.x } else { start.x },
                if start.y > end.y { end.y } else { start.y }
            );
            let maxs = Point2D::new(
                if start.x > end.x { start.x } else { end.x },
                if start.y > end.y { start.y } else { end.y }
            );
            let r = AABB::new(
                mins,
                maxs
            ).translated(instance.position.coords);
            Some(Box::new(drawables::loaders::load_rectangle(
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &r,
                filled
            )))
        }
        &GraphicElement::Circle { ref center, radius, filled, .. } => {
            Some(Box::new(drawables::loaders::load_circle(
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &(center.clone() + instance.position.coords),
                radius, filled
            )))
        },
        &GraphicElement::Pin { ref orientation, ref position, length, ref name, number, number_size, name_size, .. } => {
            Some(Box::new(drawables::loaders::load_pin(
                &(position.clone() + instance.position.coords),
                length as f32, orientation, name.clone(), number, number_size, name_size
            )))
        },
        &GraphicElement::Polygon { ref points, filled, .. } => {
            Some(Box::new(drawables::loaders::load_polygon(
                drawing::Color::new(0.61, 0.05, 0.04, 1.0),
                &points.iter().map(|point| Point2D::new(point.x, point.y) + instance.position.coords).collect(),
                filled
            )))
        },
        // &GraphicElement::TextField { ref content, ref position, ref orientation, .. } => {
        //     Some(Box::new(drawables::loaders::load_text(resource_manager, &geometry::Point2D::new(position.x, position.y), content, 30.0, orientation, component::Justify::Center, component::Justify::Center)))
        // }
        _ => None
    }
}

// pub fn field_to_drawable<'a>(resource_manager: Rc<RefCell<resource_manager::ResourceManager>>, field: &component::Field) -> Box<drawables::Drawable> {
//     Box::new(drawables::loaders::load_text(resource_manager, &geometry::Point2D::new(field.position.x, field.position.y), &field.text, field.dimension as f32, &field.orientation, field.hjustify.clone(), field.vjustify.clone()))
// }