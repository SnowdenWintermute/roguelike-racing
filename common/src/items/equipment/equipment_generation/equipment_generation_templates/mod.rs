use super::equipment_generation_template_properties::{
    EquipmentGenerationTemplate, EquipmentGenerationTemplateAffixModifiers,
    EquipmentGenerationTemplateProperties,
};
use crate::{
    combatants::CombatAttributes,
    items::equipment::{armor::ArmorCategories, weapons::DamageClassifications, EquipmentTraits},
    primatives::Range,
};
use std::collections::HashMap;
pub mod body_armor_generation_templates;
pub mod body_armor_possible_affixes;
pub mod head_gear_generation_templates;
pub mod head_gear_possible_affixes;
pub mod one_handed_melee_weapon_generation_templates;
pub mod one_handed_melee_weapon_possible_affixes;

pub struct ArmorGenerationTemplate {
    pub category: ArmorCategories,
    pub ac_range: Range<u8>,
    pub template_properties: EquipmentGenerationTemplateProperties,
}

impl ArmorGenerationTemplate {
    pub fn new(
        level_range: Range<u8>,
        ac_range: Range<u8>,
        max_durability: u8,
        category: ArmorCategories,
        requirements: HashMap<CombatAttributes, u8>,
        affix_modifiers: Option<EquipmentGenerationTemplateAffixModifiers>,
        traits: Option<Vec<EquipmentTraits>>,
    ) -> ArmorGenerationTemplate {
        ArmorGenerationTemplate {
            template_properties: EquipmentGenerationTemplateProperties {
                level_range,
                max_durability,
                requirements,
                affix_modifiers,
                traits,
            },
            ac_range,
            category,
        }
    }
}

pub struct WeaponGenerationTemplate {
    pub possbile_damage_classifications: Vec<DamageClassifications>,
    pub num_damage_classifications: u8,
    pub damage: Range<u8>,
    pub template_properties: EquipmentGenerationTemplateProperties,
}

impl WeaponGenerationTemplate {
    pub fn new(
        level_range: Range<u8>,
        damage: Range<u8>,
        max_durability: u8,
        possbile_damage_classifications: Vec<DamageClassifications>,
        num_damage_classifications: u8,
        requirements: HashMap<CombatAttributes, u8>,
        affix_modifiers: Option<EquipmentGenerationTemplateAffixModifiers>,
        traits: Option<Vec<EquipmentTraits>>,
    ) -> WeaponGenerationTemplate {
        WeaponGenerationTemplate {
            template_properties: EquipmentGenerationTemplateProperties {
                level_range,
                max_durability,
                requirements,
                affix_modifiers,
                traits,
            },
            possbile_damage_classifications,
            num_damage_classifications,
            damage,
        }
    }
}
