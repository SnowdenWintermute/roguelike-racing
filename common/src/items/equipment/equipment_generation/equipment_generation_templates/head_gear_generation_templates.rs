use super::generate_templates::generate_templates;
use super::ArmorGenerationTemplate;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::items::equipment::affixes::SuffixTypes;
use crate::items::equipment::armor_properties::ArmorCategories;
use crate::items::equipment::equipment_generation::equipment_generation_template_properties::EquipmentGenerationTemplateAffixModifiers;
use crate::items::equipment::head_gears::HeadGears;
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn head_gear_template_from_base_item(
    head_gear: &HeadGears,
    mut requirements: HashMap<CombatAttributes, u8>,
) -> ArmorGenerationTemplate {
    match head_gear {
        HeadGears::Cap => ArmorGenerationTemplate::new(
            Range::new(1, 3),
            Range::new(1, 3),
            Some(10),
            ArmorCategories::Cloth,
            Some(requirements),
            None,
            None,
        ),
        HeadGears::Bandana => ArmorGenerationTemplate::new(
            Range::new(2, 4),
            Range::new(2, 4),
            Some(12),
            ArmorCategories::Cloth,
            Some(requirements),
            None,
            None,
        ),
        HeadGears::PaddedCap => {
            requirements.insert(CombatAttributes::Intelligence, 15);
            ArmorGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(3, 6),
                Some(20),
                ArmorCategories::Cloth,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::Ribbon => ArmorGenerationTemplate::new(
            Range::new(5, 10),
            Range::new(1, 1),
            Some(20),
            ArmorCategories::Cloth,
            Some(requirements),
            Some(EquipmentGenerationTemplateAffixModifiers::new(
                None,
                Some(vec![
                    SuffixTypes::Strength,
                    SuffixTypes::Dexterity,
                    SuffixTypes::Vitality,
                    SuffixTypes::Durability,
                ]),
                None,
                Some(vec![(SuffixTypes::AllBase, 4)]),
            )),
            None,
        ),
        HeadGears::WizardHat => {
            requirements.insert(CombatAttributes::Intelligence, 35);
            ArmorGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(6, 14),
                Some(40),
                ArmorCategories::Cloth,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::Eyepatch => {
            requirements.insert(CombatAttributes::Dexterity, 5);
            ArmorGenerationTemplate::new(
                Range::new(1, 3),
                Range::new(2, 5),
                Some(14),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::LeatherHat => {
            requirements.insert(CombatAttributes::Dexterity, 15);
            ArmorGenerationTemplate::new(
                Range::new(2, 5),
                Range::new(5, 8),
                Some(20),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::LeatherHelm => {
            requirements.insert(CombatAttributes::Dexterity, 25);
            ArmorGenerationTemplate::new(
                Range::new(4, 8),
                Range::new(9, 15),
                Some(35),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::DemonsaurHelm => {
            requirements.insert(CombatAttributes::Dexterity, 45);
            ArmorGenerationTemplate::new(
                Range::new(9, 10),
                Range::new(18, 24),
                Some(45),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::Hairpin => {
            requirements.insert(CombatAttributes::Intelligence, 15);
            ArmorGenerationTemplate::new(
                Range::new(3, 4),
                Range::new(2, 2),
                Some(20),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::Skullcap => {
            requirements.insert(CombatAttributes::Strength, 20);
            requirements.insert(CombatAttributes::Dexterity, 20);
            ArmorGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(8, 16),
                Some(28),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::Coif => {
            requirements.insert(CombatAttributes::Strength, 30);
            requirements.insert(CombatAttributes::Intelligence, 30);
            ArmorGenerationTemplate::new(
                Range::new(4, 8),
                Range::new(20, 26),
                Some(36),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::OhmushellMask => {
            requirements.insert(CombatAttributes::Strength, 30);
            requirements.insert(CombatAttributes::Dexterity, 30);
            requirements.insert(CombatAttributes::Intelligence, 30);
            ArmorGenerationTemplate::new(
                Range::new(7, 10),
                Range::new(30, 38),
                Some(50),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::Circlet => ArmorGenerationTemplate::new(
            Range::new(2, 5),
            Range::new(5, 10),
            Some(30),
            ArmorCategories::Plate,
            Some(requirements),
            None,
            None,
        ),
        HeadGears::Crown => ArmorGenerationTemplate::new(
            Range::new(3, 7),
            Range::new(10, 20),
            Some(35),
            ArmorCategories::Plate,
            Some(requirements),
            None,
            None,
        ),
        HeadGears::FullHelm => {
            requirements.insert(CombatAttributes::Strength, 30);
            ArmorGenerationTemplate::new(
                Range::new(5, 10),
                Range::new(22, 30),
                Some(40),
                ArmorCategories::Plate,
                Some(requirements),
                None,
                None,
            )
        }
        HeadGears::GreatHelm => {
            requirements.insert(CombatAttributes::Strength, 55);
            ArmorGenerationTemplate::new(
                Range::new(9, 10),
                Range::new(32, 40),
                Some(50),
                ArmorCategories::Plate,
                Some(requirements),
                None,
                None,
            )
        }
    }
}

pub static HEAD_GEAR_GENERATION_TEMPLATES: Lazy<HashMap<HeadGears, ArmorGenerationTemplate>> =
    Lazy::new(|| generate_templates(head_gear_template_from_base_item));

pub static HEAD_GEARS_BY_LEVEL: Lazy<HashMap<u8, Vec<HeadGears>>> = Lazy::new(|| {
    let items_and_level_ranges: Vec<(&HeadGears, &Range<u8>)> = HEAD_GEAR_GENERATION_TEMPLATES
        .iter()
        .collect::<Vec<(&HeadGears, &ArmorGenerationTemplate)>>()
        .iter()
        .map(|template| (template.0, &template.1.template_properties.level_range))
        .collect::<Vec<(&HeadGears, &Range<u8>)>>();
    items_by_level(items_and_level_ranges)
});
