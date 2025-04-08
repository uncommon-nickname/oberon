use std::any::type_name;
use std::cell::RefCell;

use crate::ecs::world::World;

#[derive(Debug)]
pub struct Entity<T>
{
    pub id: usize,
    pub item: RefCell<T>,
}

pub struct EntityBuilder<'a>
{
    id: usize,
    world: &'a mut World,
}

impl<T> Entity<T>
{
    pub const fn new(id: usize, item: T) -> Self
    {
        let item = RefCell::new(item);
        Self { id, item }
    }
}

impl<'a> EntityBuilder<'a>
{
    pub fn new(id: usize, world: &'a mut World) -> Self
    {
        Self { id, world }
    }

    pub fn with<T: 'static>(self, component: T) -> Self
    {
        if let Some(storage) = self.world.get_storage_mut::<T>()
        {
            storage.add(self.id, component);
            return self;
        }
        panic!(
            "You cannot spawn an entity `{}` with unregistered type `{}`.",
            self.id,
            type_name::<T>()
        );
    }

    #[inline]
    pub fn into_id(self) -> usize
    {
        self.id
    }
}
