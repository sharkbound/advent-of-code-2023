use std::borrow::Borrow;

pub struct DerefItemIter<Iterator> {
    iter: Iterator,
}

impl<Iter, Item> Iterator for DerefItemIter<Iter>
    where Iter: Iterator<Item=Item>, Item: Borrow<Item> + Copy {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| *item.borrow())
    }
}

pub trait DerefItems<T>: Sized {
    fn deref_items(self) -> DerefItemIter<Self>;
}

impl<It, T> DerefItems<T> for It
    where It: Iterator<Item=T> {
    fn deref_items(self) -> DerefItemIter<Self> {
        DerefItemIter { iter: self }
    }
}