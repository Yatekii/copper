use std::f32;
use std::cell::Cell;

use uuid::Uuid;

use nom::types::CompleteByteSlice;
use geometry::schema_elements::*;
use utils::traits::clone_cached_aabb;

use geometry::{ Point2D, AABB };
use parsing::component::parse_component;
use state::schema::component_instance::ComponentInstance;

#[derive(Debug, PartialEq, Clone)]
pub enum OptionFlag {
    Normal,
    Power
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct Component {
    pub uuid: Uuid,
    pub name: String,
    pub reference: String,
    pub text_offset: isize,
    pub draw_pin_number: bool,
    pub draw_pin_name: bool,
    pub unit_count: isize,
    pub units_locked: bool,
    pub option_flag: OptionFlag,
    pub fields: Vec<Field>,
    pub alias: Vec<String>,
    pub graphic_elements: Vec<GraphicElement>,
    pub pins: Vec<PinDescription>,
    #[derivative(Debug="ignore", Clone(clone_with="clone_cached_aabb"))]
    pub bounding_box: Cell<Option<AABB>>
}

impl Component {
    pub fn parse(input: CompleteByteSlice) -> Option<Component> {
        let parse_res = parse_component(input);

        match parse_res {
            Ok((_, o)) => Some(o),
            _ => None
        }
    }

    pub fn instantiate(&self) -> ComponentInstance {
        ComponentInstance::new(self.name.clone())
    }

    pub fn get_graphic_elements(&self) -> &Vec<GraphicElement> {
        &self.graphic_elements
    }

    pub fn set_graphic_elements(&mut self, elements: Vec<GraphicElement>) {
        self.graphic_elements = elements;
    }

    pub fn add_graphic_element(&mut self, element: GraphicElement) {
        self.graphic_elements.push(element);
    }

    pub fn add_graphic_elements(&mut self, elements: &mut Vec<GraphicElement>) {
        self.graphic_elements.append(elements);
    }

    pub fn update_boundingbox(&self) {
        let mut max_x = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;
        for shape in &self.graphic_elements {
            match shape {
                &GraphicElement::Rectangle { ref start, ref end, .. } => {
                    max_x = max_x.max(start.x).max(end.x);
                    min_x = min_x.min(start.x).min(end.x);
                    max_y = max_y.max(start.y).max(end.y);
                    min_y = min_y.min(start.y).min(end.y);
                },
                &GraphicElement::Circle { ref center, radius, .. } => {
                    max_x = max_x.max(center.x + radius);
                    min_x = min_x.min(center.x - radius);
                    max_y = max_y.max(center.y + radius);
                    min_y = min_y.min(center.y - radius);
                },
                &GraphicElement::Pin { ref position, ref orientation, length, .. } => {
                    let end_position = position.clone() + (orientation.unit_vec() * (length as f32));
                    max_x = max_x.max(position.x).max(end_position.x );
                    min_x = min_x.min(position.x).min(end_position.x);
                    max_y = max_y.max(position.y).max(end_position.y);
                    min_y = min_y.min(position.y).min(end_position.y);
                },
                &GraphicElement::Polygon { ref points, .. } => {
                    for p in points {
                        max_x = max_x.max(p.x);
                        min_x = min_x.min(p.x);
                        max_y = max_y.max(p.y);
                        min_y = min_y.min(p.y);
                    }
                }
                _ => ()
            }
        }

        for field in &self.fields {
            if field.visible {
                max_x = max_x.max(field.position.x + field.dimension as f32 / 2.0);
                min_x = min_x.min(field.position.x - field.dimension as f32 / 2.0);
                max_y = max_y.max(field.position.y + field.dimension as f32 / 2.0);
                min_y = min_y.min(field.position.y - field.dimension as f32 / 2.0);
            }
        }

        if max_x > f32::MIN
        && max_y > f32::MIN
        && min_x < f32::MAX
        && min_y < f32::MAX {
            self.bounding_box.set(Some(AABB::new(
                Point2D::new(min_x, min_y),
                Point2D::new(max_x, max_y)
            )))
        } else {
            self.bounding_box.set(Some(AABB::new(
                Point2D::new(0.0, 0.0),
                Point2D::new(0.0, 0.0)
            )))
        }
    }

    pub fn get_boundingbox(&self) -> AABB {
        use utils::traits::CellCopy;
        self.bounding_box.copy().take().unwrap_or_else(|| {
            self.update_boundingbox();
            // Unwrap is always safe as we just calculated a BB
            self.bounding_box.copy().take().unwrap()
        })
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub n: isize,
    pub text: String,
    pub position: Point2D,
    pub dimension: usize,
    pub orientation: TextOrientation,
    pub visible: bool,
    pub hjustify: Justify,
    pub vjustify: Justify,
    pub italic: bool,
    pub bold: bool,
    pub name: Option<String>,
}