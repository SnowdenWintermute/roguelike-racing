use crate::combatants::abilities::{TargetingScheme, ValidTargets};

use super::{consumables::ConsumableTypes, ConsumableProperties, Item, ItemProperties};
use rand::prelude::*;
use std::vec;
use strum::IntoEnumIterator;

impl Item {
    pub fn generate_consumable_properties(level: u16) -> ItemProperties {
        let consumable_types: Vec<_> = ConsumableTypes::iter().collect();
        let consumable_type = *consumable_types.choose(&mut rand::thread_rng()).unwrap();
        let uses_remaining = 1;

        ItemProperties::Consumable(ConsumableProperties {
            consumable_type,
            uses_remaining,
            combat_use_only: true,
            requires_combat_turn: true,
            targeting_schemes: vec![TargetingScheme::Single],
            valid_targets: ValidTargets::AllyOrSelf,
        })
    }
}
