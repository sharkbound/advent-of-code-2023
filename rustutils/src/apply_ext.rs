pub trait ApplyImmutableExt<T, F>
where
    F: Fn(&T),
{
    fn apply(&self, handler: F) -> &Self;
}

impl<T, F> ApplyImmutableExt<T, F> for T
where
    F: Fn(&T),
{
    #[inline]
    fn apply(&self, handler: F) -> &Self {
        handler(self);
        self
    }
}

pub trait ApplyMutExt<T, F>
where
    F: FnMut(&mut T),
{
    fn apply_mut(&mut self, handler: F) -> &mut Self;
}

impl<T, F> ApplyMutExt<T, F> for T
where
    F: FnMut(&mut T),
{
    #[inline]
    fn apply_mut(&mut self, mut handler: F) -> &mut Self {
        handler(self);
        self
    }
}

pub trait ApplyOwnExt<T, F>
where
    F: FnOnce(T) -> T,
{
    fn apply_own(self, handler: F) -> Self;
}

impl<T, F> ApplyOwnExt<T, F> for T
where
    F: FnOnce(T) -> T,
{
    #[inline]
    fn apply_own(self, handler: F) -> Self {
        handler(self)
    }
}
