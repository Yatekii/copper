use std::cell::Cell;
use ncollide2d::bounding_volume::{ AABB };
use ncollide2d::math::{ Vector };

pub type SchemaAABB = AABB<f32>;

pub trait Translatable {
    fn translated(&self, distance: Vector<f32>) -> SchemaAABB;
}

impl Translatable for SchemaAABB {
    fn translated(&self, distance: Vector<f32>) -> SchemaAABB {
        SchemaAABB::new(self.mins() + distance, self.maxs() + distance)
    }
}

// This trait is a little hack to use Cell for a non copiable value.
// We steal the value, clone it and put it back in.

pub trait CellCopy<T> {
    fn copy(&self) -> Cell<T>;
}

impl<T> CellCopy<T> for Cell<T> where T: Clone + Default {
    fn copy(&self) -> Cell<T> {
        let v = self.take();
        let v2 = v.clone();
        self.set(v);
        Cell::new(v2)
    }
}

pub fn clone_cached_aabb(source: &Cell<Option<SchemaAABB>>) -> Cell<Option<SchemaAABB>> {
    source.copy()
}