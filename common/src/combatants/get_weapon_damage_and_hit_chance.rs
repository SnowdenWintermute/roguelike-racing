use super::CombatantProperties;
use crate::app_consts::OFF_HAND_ACCURACY_MODIFIER;
use crate::app_consts::OFF_HAND_DAMAGE_MODIFIER;
use crate::items::equipment::trait_effects::get_weapon_percent_damage_increase_trait_damage_modifier::get_weapon_percent_damage_increase_trait_damage_modifier;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentTraits;
use crate::primatives::Range;

impl CombatantProperties {
    pub fn get_weapon_damage_and_hit_chance(
        weapon_properties: &WeaponProperties,
        traits: &Option<Vec<EquipmentTraits>>,
        combatant_base_damage: u16,
        accuracy: u16,
        is_off_hand: bool,
    ) -> (Range<u16>, u16) {
        let percent_damage_increase_from_trait =
            get_weapon_percent_damage_increase_trait_damage_modifier(traits);
        let mut modified_min = weapon_properties.damage.min as f32 + combatant_base_damage as f32;
        let mut modified_max = weapon_properties.damage.max as f32 + combatant_base_damage as f32;
        modified_min *= percent_damage_increase_from_trait;
        modified_max *= percent_damage_increase_from_trait;
        let mut modified_acc = accuracy as f32;

        if is_off_hand {
            modified_min *= OFF_HAND_DAMAGE_MODIFIER;
            modified_max *= OFF_HAND_DAMAGE_MODIFIER;
            modified_acc *= OFF_HAND_ACCURACY_MODIFIER;
        }

        (
            Range::new(modified_min as u16, modified_max as u16),
            modified_acc as u16,
        )
    }
}
