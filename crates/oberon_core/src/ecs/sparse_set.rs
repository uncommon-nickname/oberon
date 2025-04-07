use std::any::Any;
use std::cell::RefCell;

pub trait ComponentStorage
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn delete(&mut self, id: usize);
}

#[derive(Debug)]
pub struct Entity<T>
{
    pub id: usize,
    pub item: RefCell<T>,
}

#[derive(Debug)]
pub struct SparseSet<T>
{
    dense: Vec<Entity<T>>,
    sparse: Vec<Option<usize>>,
}

impl<T> Entity<T>
{
    pub fn new(id: usize, item: T) -> Self
    {
        let item = RefCell::new(item);
        Self { id, item }
    }
}

impl<T: 'static> ComponentStorage for SparseSet<T>
{
    #[inline]
    fn as_any(&self) -> &dyn Any
    {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self
    }

    fn delete(&mut self, id: usize)
    {
        if let Some(index) = self.sparse[id]
        {
            self.dense.swap_remove(index);

            if index < self.dense.len()
            {
                let moved = &self.dense[index];
                self.sparse[moved.id] = Some(index);
            }
            self.sparse[id] = None;
        }
    }
}

impl<T> SparseSet<T>
{
    pub fn new(size: usize) -> Self
    {
        Self {
            dense: Vec::with_capacity(size),
            sparse: vec![None; size],
        }
    }

    pub fn add(&mut self, id: usize, item: T)
    {
        if let Some(stored) = self.get(id)
        {
            stored.item.replace(item);
            return;
        }

        let index = Some(self.dense.len());

        self.dense.push(Entity::new(id, item));
        self.sparse[id] = index;
    }

    pub fn get(&self, id: usize) -> Option<&Entity<T>>
    {
        let index = self.sparse[id]?;
        let entry = &self.dense[index];

        Some(entry)
    }

    #[inline]
    pub fn get_all(&self) -> &[Entity<T>]
    {
        self.dense.as_slice()
    }

    #[cfg(test)]
    pub fn contains(&self, id: usize) -> bool
    {
        self.sparse[id].is_some()
    }

    #[cfg(test)]
    pub fn size(&self) -> usize
    {
        self.dense.len()
    }
}

#[cfg(test)]

mod tests
{
    use super::*;

    #[test]
    #[should_panic]
    fn crash_when_adding_more_than_capacity_would_fit()
    {
        let mut set = SparseSet::new(1);

        set.add(0, 1);
        set.add(1, 2);
    }

    #[test]
    fn add_new_element()
    {
        let mut set = SparseSet::new(10);

        set.add(5, 1);

        assert_eq!(set.sparse[5].unwrap(), 0);
        assert_eq!(set.dense[0].item, RefCell::new(1));
    }

    #[test]
    fn add_element_which_already_exists()
    {
        let mut set = SparseSet::new(10);

        set.add(5, 1);
        set.add(5, 2);

        assert_eq!(set.sparse[5].unwrap(), 0);
        assert_eq!(set.dense[0].item, RefCell::new(2));
    }

    #[test]
    fn contains_returns_true_if_id_in_sparse()
    {
        let mut set = SparseSet::new(10);

        set.add(0, 1);

        assert_eq!(set.contains(0), true);
    }

    #[test]
    fn contains_returns_false_if_id_not_in_sparse()
    {
        let mut set = SparseSet::new(2);

        set.add(0, 1);

        assert_eq!(set.contains(1), false);
    }

    #[test]
    fn delete_does_nothing_when_not_exists()
    {
        let mut set = SparseSet::new(2);

        set.add(0, 1);
        set.delete(1);

        assert_eq!(set.contains(0), true);
    }

    #[test]
    fn delete_when_only_one_element()
    {
        let mut set = SparseSet::new(10);

        set.add(4, 1);
        set.delete(4);

        assert_eq!(set.contains(4), false);
    }

    #[test]
    fn delete_swaps_with_last_element()
    {
        let mut set = SparseSet::new(10);

        set.add(4, 1);
        set.add(7, 2);
        set.delete(4);

        assert_eq!(set.contains(4), false);
        assert_eq!(set.get(7).unwrap().item, RefCell::new(2));
        assert_eq!(set.sparse[7].unwrap(), 0);
    }

    #[test]
    fn get_returns_null_when_not_present()
    {
        let mut set = SparseSet::new(2);

        set.add(0, 1);

        assert_eq!(set.get(1).is_none(), true);
    }

    #[test]
    fn get_returns_ref_when_present()
    {
        let mut set = SparseSet::new(1);

        set.add(0, 1);

        assert_eq!(set.get(0).unwrap().item, RefCell::new(1));
    }

    #[test]
    fn size_returns_current_set_size()
    {
        let mut set = SparseSet::new(10);

        set.add(0, 1);
        set.add(1, 2);

        assert_eq!(set.size(), 2);
    }
}
