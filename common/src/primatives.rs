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
