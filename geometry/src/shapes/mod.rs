use std::any::Any;

pub mod simple;
pub mod affine;


pub trait Shape {
}
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + Shape> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait AnyShape: Shape + AsAny {}
impl<T: Shape + AsAny> AnyShape for T {}