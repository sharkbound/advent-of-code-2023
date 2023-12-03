use std::rc::Rc;
use std::sync::Arc;

trait IntoARC<T> {
    fn into_arc(self) -> Arc<T>;
}

impl<T> IntoARC<T> for T {
    fn into_arc(self) -> Arc<T> {
        Arc::new(self)
    }
}

trait IntoRC<T> {
    fn into_rc(self) -> Rc<T>;
}

impl<T> IntoRC<T> for T {
    fn into_rc(self) -> Rc<T> {
        Rc::new(self)
    }
}

trait IntoBox<T> {
    fn into_box(self) -> Box<T>;
}

impl<T> IntoBox<T> for T {
    fn into_box(self) -> Box<T> {
        Box::new(self)
    }
}
