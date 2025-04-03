use std::any::Any;

pub trait ComponentStorage
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn delete(&mut self, id: usize);
}

#[derive(Debug)]
pub struct Entry<T>
{
    pub id: usize,
    pub item: T,
}

#[derive(Debug)]
pub struct SparseSet<T>
{
    dense: Vec<Entry<T>>,
    sparse: Vec<Option<usize>>,
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
        if let Some(stored) = self.get_mut(id)
        {
            *stored = item;
            return;
        }

        let index = Some(self.dense.len());

        self.dense.push(Entry { id, item });
        self.sparse[id] = index;
    }

    pub fn contains(&self, id: usize) -> bool
    {
        self.sparse[id].is_some()
    }

    pub fn clear(&mut self)
    {
        self.dense.clear();
        self.sparse.fill(None);
    }

    pub fn get(&self, id: usize) -> Option<&T>
    {
        let index = self.sparse[id]?;
        let entry = &self.dense[index];

        Some(&entry.item)
    }

    #[inline]
    pub fn get_all(&self) -> &[Entry<T>]
    {
        self.dense.as_slice()
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T>
    {
        let index = self.sparse[id]?;
        let entry = &mut self.dense[index];

        Some(&mut entry.item)
    }

    #[inline]
    pub fn get_all_mut(&mut self) -> &mut [Entry<T>]
    {
        self.dense.as_mut_slice()
    }

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
        assert_eq!(set.dense[0].item, 1);
    }

    #[test]
    fn add_element_which_already_exists()
    {
        let mut set = SparseSet::new(10);

        set.add(5, 1);
        set.add(5, 2);

        assert_eq!(set.sparse[5].unwrap(), 0);
        assert_eq!(set.dense[0].item, 2);
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
    fn clear_removes_values_from_dense_and_resets_state()
    {
        let mut set = SparseSet::new(10);

        set.add(0, 1);
        set.add(1, 2);
        set.clear();

        assert_eq!(set.contains(0), false);
        assert_eq!(set.contains(1), false);
        assert_eq!(set.dense.is_empty(), true);
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
        assert_eq!(*set.get(7).unwrap(), 2);
        assert_eq!(set.sparse[7].unwrap(), 0);
    }

    #[test]
    fn get_mut_returns_null_when_not_present()
    {
        let mut set = SparseSet::new(2);

        set.add(0, 1);

        assert_eq!(set.get_mut(1).is_none(), true);
    }

    #[test]
    fn get_mut_returns_mutable_ref_when_present()
    {
        let mut set = SparseSet::new(1);

        set.add(0, 1);

        assert_eq!(*set.get_mut(0).unwrap(), 1);
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

        assert_eq!(*set.get(0).unwrap(), 1);
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
