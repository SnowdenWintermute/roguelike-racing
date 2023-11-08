use super::equipment_generation_template_properties::EquipmentGenerationTemplateProperties;
use crate::items::equipment::affixes::PrefixTypes;
use crate::items::equipment::affixes::SuffixTypes;
use crate::items::equipment::EquipmentTypes;
pub mod body_armor_blueprints;
pub mod head_gear_blueprints;
pub mod jewelry_blueprints;
pub mod one_handed_melee_weapon_blueprints;
pub mod shield_blueprints;
pub mod two_handed_melee_weapon_blueprints;
pub mod two_handed_ranged_weapon_blueprints;

pub struct EquipmentBlueprint<'a> {
    pub equipment_type: EquipmentTypes,
    pub template_properties: EquipmentGenerationTemplateProperties,
    pub possible_prefixes: Vec<&'a (PrefixTypes, u8)>,
    pub possible_suffixes: Vec<&'a (SuffixTypes, u8)>,
}

impl<'a> EquipmentBlueprint<'a> {
    pub fn new(
        equipment_type: EquipmentTypes,
        template_properties: EquipmentGenerationTemplateProperties,
        possible_prefixes: Vec<&'a (PrefixTypes, u8)>,
        possible_suffixes: Vec<&'a (SuffixTypes, u8)>,
    ) -> EquipmentBlueprint<'a> {
        EquipmentBlueprint {
            equipment_type,
            template_properties,
            possible_prefixes,
            possible_suffixes,
        }
    }
}
