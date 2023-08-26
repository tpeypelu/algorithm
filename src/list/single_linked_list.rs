use std::boxed::Box;
use std::cmp::PartialEq;

use crate::list::List;

#[derive(Debug)]
pub struct SingleLinkedList<T> {
    value: T,
    next: Option<Box<SingleLinkedList<T>>>,
}

impl<T> SingleLinkedList<T> {
    pub fn new(value: T) -> Self {
        Self{
            value: value,
            next: None,
        }
    }

    pub fn add(&mut self, list: Self) {
        match &mut self.next {
            None => self.next = Some(Box::new(list)),
            Some(ref mut x) => x.add(list),
        }
    }
}

impl<T> List<T> for SingleLinkedList<T> {
    fn next(&self) -> &Option<Box<Self>> {
        &self.next
    }

    fn value(&self) -> &T {
        &self.value
    }
}

impl<T: PartialEq> PartialEq for SingleLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.next, &other.next) {
            (None, None) => self.value == other.value,
            (Some(x), Some(y)) => x == y,
            _ => false,
        }
    }
}
