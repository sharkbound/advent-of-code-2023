pub trait MapToExt<In, Out, F>
where
    F: FnOnce(In) -> Out,
{
    fn map_to(self, f: F) -> Out;
}

impl<In, Out, F: FnOnce(In) -> Out> MapToExt<In, Out, F> for In {
    #[inline]
    fn map_to(self, f: F) -> Out {
        f(self)
    }
}
