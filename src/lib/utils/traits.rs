use std::cell::Cell;


use geometry::{ Vector2D, AABB };


pub trait Translatable {
    fn translated(&self, distance: Vector2D) -> AABB;
}

impl Translatable for AABB {
    fn translated(&self, distance: Vector2D) -> AABB {
        AABB::new(self.mins() + distance, self.maxs() + distance)
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

pub fn clone_cached_aabb(source: &Cell<Option<AABB>>) -> Cell<Option<AABB>> {
    source.copy()
}