use uuid::Uuid;
use std::collections::HashSet;
use std::collections::hash_set;
use std::iter::{
    Iterator,
    IntoIterator,
};
use state::component_libraries::ComponentLibraries;
use state::schema::Schema;
use geometry::*;

/// An `ItemGroup` holds a set of items that are selected on the schema. This can be Components or Wires.
/// Each item is represented through an `Uuid` and can only be contained once in each group. It can be contained by multiple groups tho.
/// Internally a `HashSet` is used to ensure uniqueness.
/// This struct is intended to be used for enabling the selection and marking of items on the schema.

#[derive(Clone)]
pub struct ItemGroup {
    items: HashSet<Uuid>
}

impl ItemGroup {
    /// Creates a new, empty `ItemGroup`.
    pub fn new() -> ItemGroup {
        ItemGroup {
            items: HashSet::new()
        }
    }

    /// Returns all the items contained in the group as a mutable `HashSet`.
    pub fn get_items_mut(&mut self) -> &HashSet<Uuid> {
        &self.items
    }

    /// Returns all the items contained in the group as a `HashSet`.
    pub fn get_items(&self) -> &HashSet<Uuid> {
        &self.items
    }

    /// Adds a new items to the group. If the item is already contained, nothing happens.
    pub fn insert(&mut self, item: Uuid) {
        self.items.insert(item);
    }

    /// Removes an item from the group. If the item is not contained, nothing happens.
    pub fn remove(&mut self, item: &Uuid) {
        self.items.remove(item);
    }

    /// Removes an item from the group. If the item is not contained, nothing happens.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns true if the set is empty.
    pub fn is_empty(&mut self) -> bool {
        self.items.is_empty()
    }

    /// Replaces the entire item set with a new one.
    pub fn set_items(&mut self, items: HashSet<Uuid>) {
        self.items = items;
    }

    /// Takes a list of item `Uuid`s and returns their outer `AABB`.
    pub fn get_grouped_component_aabb(&self, libraries: &ComponentLibraries, schema: &Schema) -> Option<AABB> {
        let mut aabb = None;
        for uuid in &self.items {
            let instance = schema.get_component_instance(&uuid);
            let component = libraries.get_component_by_name(&instance.name);
            if let Some(c) = component {
                let bb = instance.get_boundingbox(c).clone();
                use ncollide2d::bounding_volume::BoundingVolume;
                if aabb.is_none() {
                    aabb = Some(bb);
                } else {
                    // unwrap() here is safe as we checked for is_none()
                    aabb.as_mut().unwrap().merge(&bb);
                }
            }
        }
        aabb
    }

    pub fn iter(&self) -> Iter {
        Iter { iter: self.items.iter() }
    }
}

pub struct Iter<'a> {
    iter: hash_set::Iter<'a, Uuid>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Uuid;

    fn next(&mut self) -> Option<&'a Uuid> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> IntoIterator for &'a ItemGroup {
    type Item = &'a Uuid;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}