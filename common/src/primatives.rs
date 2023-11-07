use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityProperties {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MaxAndCurrent<T> {
    pub max: T,
    pub current: T,
}

impl<T> MaxAndCurrent<T> {
    pub fn new(max: T, current: T) -> MaxAndCurrent<T> {
        MaxAndCurrent { max, current }
    }
}

#[derive(PartialEq)]
pub enum UpOrDown {
    Up,
    Down,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<T> Range<T> {
    pub fn new(min: T, max: T) -> Range<T> {
        Range { min, max }
    }
}
