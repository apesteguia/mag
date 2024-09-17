use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos<T>
where
    T: Display + Clone + Ord,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
