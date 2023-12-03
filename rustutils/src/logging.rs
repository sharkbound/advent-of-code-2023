pub trait DebugLog {
    fn debug_expanded(&self) -> &Self;
    fn debug_expanded_owned(self) -> Self;
    fn debug(&self) -> &Self;
    fn debug_owned(self) -> Self;
}

impl<T> DebugLog for T
where
    T: std::fmt::Debug,
{
    fn debug_expanded(&self) -> &Self {
        println!("{:#?}", self);
        self
    }

    fn debug_expanded_owned(self) -> Self {
        self.debug_expanded();
        self
    }

    fn debug(&self) -> &Self {
        println!("{:?}", self);
        self
    }

    fn debug_owned(self) -> Self {
        self.debug();
        self
    }
}

pub trait DisplayLog {
    fn display(&self) -> &Self;
    fn display_owned(self) -> Self;
}

impl<T> DisplayLog for T
where
    T: std::fmt::Display,
{
    fn display(&self) -> &Self {
        println!("{}", self);
        self
    }

    fn display_owned(self) -> Self {
        self.display();
        self
    }
}
