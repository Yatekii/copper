mod drawable_component;
mod drawable_wire;

use std::fs;
use std::rc::Rc;
use std::cell::Cell;
use std::f32::consts::PI;

use ncollide2d::partitioning::DBVT;
use ncollide2d::partitioning::DBVTLeaf;
use ncollide2d::query::PointInterferencesCollector;

use manipulation::library::Library;
pub use self::drawable_component::DrawableComponent;
pub use self::drawable_wire::DrawableWire;
use geometry::schema_elements::*;
use drawing::view_state::ViewState;

use parsing::schema_file::ComponentInstance;

use geometry::{
    Point2D,
    Vector2D,
    Vector3,
    Matrix4,
    AABB
};
use drawing;

pub struct DrawableComponentInstance {
    pub instance: ComponentInstance,
    pub drawable: Rc<DrawableComponent>,
}

impl DrawableComponentInstance {
    pub fn draw(&self, buffers: &mut drawing::Buffers) {
        self.drawable.draw(buffers, &self.instance);
    }

    pub fn get_boundingbox(&self) -> AABB {
        let i = &self.instance;
        use utils::traits::Translatable;
        let bb = i.get_boundingbox().translated(Vector2D::new(i.position.x, i.position.y));
        bb
    }
}

/// Represents a schema containing all its components and necessary resource references
pub struct Schema {
    wires: Vec<DrawableWire>,
    drawables: Vec<DrawableComponentInstance>,
    bounding_box: Cell<Option<AABB>>,
    collision_world: DBVT<f32, u32, AABB>,
}

impl Schema {
    /// Creates a new blank schema
    pub fn new() -> Schema {
        Schema {
            wires: Vec::new(),
            drawables: Vec::new(),
            bounding_box: Cell::new(None),
            collision_world: DBVT::new(),
        }
    }

    /// Populates a schema from a schema file pointed to by <path>.
    pub fn load(&mut self, library: &Library, path: String) {
        if let Ok(mut file) = fs::File::open(path) {
            if let Some(mut schema_file) = ::parse_schema(&mut file) {
                let mut component_count = 0;
                for mut instance in schema_file.components {
                    let component = library.get_component(&instance);
                    instance.set_component(component.clone());

                    let drawable_component = DrawableComponentInstance {
                        instance: instance.clone(),
                        drawable: Rc::new(DrawableComponent::new(component_count, component.clone())),
                    };
                    let aabb = instance.get_boundingbox().clone();
                    let _ = self.collision_world.insert(DBVTLeaf::new(aabb, component_count));
                    component_count += 1;

                    self.drawables.push(drawable_component);
                }

                let wires = schema_file.wires.iter().map( |w: &WireSegment| {
                    let dw = DrawableWire::from_schema(component_count, w);
                    let aabb = dw.get_boundingbox().clone();
                    let _ = self.collision_world.insert(DBVTLeaf::new(aabb, component_count));
                    component_count += 1;
                    dw
                }).collect::<Vec<DrawableWire>>();

                self.wires.extend(wires);
            } else {
                println!("Could not parse the library file.");
            }
        } else {
            println!("Lib file could not be opened.");
        }
    }

    /// Issues draw calls to render the entire schema
    pub fn draw(&self, buffers: &mut drawing::Buffers) {
        for drawable in &self.drawables {
            // Unwrap should be ok as there has to be an instance for every component in the schema

            drawable.draw(buffers);
        }

        for wire in &self.wires {
            wire.draw(buffers);
        }
    }

    /// This function infers the bounding box containing all boundingboxes of the objects contained in the schema
    pub fn get_bounding_box(&self) -> AABB {
        let mut aabb = AABB::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(0.0, 0.0)
        );
        for component in &self.drawables {
            let i = &component.instance;
            use utils::traits::Translatable;
            let bb = &i.get_boundingbox().translated(Vector2D::new(i.position.x, i.position.y));
            use ncollide2d::bounding_volume::BoundingVolume;
            aabb.merge(bb);
        }
        aabb
    }

    pub fn get_currently_hovered_component_id(&self, cursor: Point2D) -> Option<u32> {
        let mut result = Vec::new();
        {
            let mut visitor = PointInterferencesCollector::new(&cursor, &mut result);
            self.collision_world.visit(&mut visitor);
        }
        result.first().map(|i| *i)
    }

    pub fn get_drawable_component_instance_by_id(&self, component_id: u32) -> &DrawableComponentInstance {
        &self.drawables[component_id as usize]
    }

    pub fn get_currently_selected_component(&self) -> Option<&DrawableComponentInstance> {
        for component in &self.drawables {
            let i = &component.instance;
            use utils::traits::Translatable;
            let bb = &i.get_boundingbox().translated(Vector2D::new(i.position.x, i.position.y));
            if bb.half_extents().x > 0.0 {
                return Some(component);
            }
        }
        None
    }

    pub fn rotate_hovered_component(&mut self, view_state: &ViewState) {
        view_state.hovered_component_id.map(|cc| {
            self.rotate_component(cc);
        });
    }

    pub fn rotate_component(&mut self, component_id: u32) {
        self.drawables[component_id as usize].instance.rotation *= Matrix4::from_axis_angle(
            &Vector3::z_axis(),
            PI / 2.0
        );
    }
}