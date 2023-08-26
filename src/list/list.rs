use std::boxed::Box;

pub trait List<T> {
    fn next(&self) -> &Option<Box<Self>>;

    fn value(&self) -> &T;
}
