use core::fmt;
use serde::Deserialize;
use serde::Serialize;

pub type EntityId = u32;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityProperties {
    pub id: EntityId,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponSlot {
    MainHand,
    OffHand,
}

impl fmt::Display for WeaponSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WeaponSlot::MainHand => write!(f, "Main Hand"),
            WeaponSlot::OffHand => write!(f, "Off Hand"),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum GainedOrLost {
    Gained,
    Lost,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub enum NextOrPrevious {
    Next,
    Previous,
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
