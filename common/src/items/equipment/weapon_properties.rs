use crate::combat::hp_change_source_types::HpChangeSource;
use crate::primatives::Range;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct WeaponProperties {
    pub damage_classifications: Vec<HpChangeSource>,
    pub damage: Range<u8>,
}

impl WeaponProperties {
    pub fn new(damage_classifications: Vec<HpChangeSource>, damage: Range<u8>) -> WeaponProperties {
        WeaponProperties {
            damage_classifications,
            damage,
        }
    }
}
