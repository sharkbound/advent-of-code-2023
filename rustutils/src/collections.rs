use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;

pub trait CollectToVec<T, I>: Sized
where
    T: Iterator<Item = I>,
{
    fn collect_to_vec(self) -> Vec<I>;
    fn collect_to_vecdeque(self) -> VecDeque<I>;
}

impl<T, I> CollectToVec<T, I> for T
where
    T: Iterator<Item = I>,
{
    fn collect_to_vec(self) -> Vec<I> {
        self.collect()
    }

    fn collect_to_vecdeque(self) -> VecDeque<I> {
        self.collect()
    }
}

pub trait CollectToHashSet<T, I>: Sized
where
    T: Iterator<Item = I>,
    I: Eq + Hash,
{
    fn collect_to_hashset(self) -> HashSet<I>;
}

impl<T, I> CollectToHashSet<T, I> for T
where
    T: Iterator<Item = I>,
    I: Eq + Hash,
{
    fn collect_to_hashset(self) -> HashSet<I> {
        self.collect()
    }
}
