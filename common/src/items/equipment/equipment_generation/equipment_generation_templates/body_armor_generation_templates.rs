use super::generate_templates::generate_templates;
use super::ArmorGenerationTemplate;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::items::equipment::armor_properties::ArmorCategories;
use crate::items::equipment::body_armors::BodyArmors;
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn body_armor_template_from_base_item(
    armor: &BodyArmors,
    mut requirements: HashMap<CombatAttributes, u8>,
) -> ArmorGenerationTemplate {
    match armor {
        BodyArmors::Rags => ArmorGenerationTemplate::new(
            Range::new(1, 3),
            Range::new(2, 6),
            Some(6),
            ArmorCategories::Cloth,
            Some(requirements),
            None,
            None,
        ),
        BodyArmors::Cape => ArmorGenerationTemplate::new(
            Range::new(1, 4),
            Range::new(5, 10),
            Some(12),
            ArmorCategories::Cloth,
            Some(requirements),
            None,
            None,
        ),
        BodyArmors::Cloak => ArmorGenerationTemplate::new(
            Range::new(3, 7),
            Range::new(10, 14),
            Some(18),
            ArmorCategories::Cloth,
            Some(requirements),
            None,
            None,
        ),
        BodyArmors::Robe => {
            requirements.insert(CombatAttributes::Intelligence, 5);
            ArmorGenerationTemplate::new(
                Range::new(6, 9),
                Range::new(18, 22),
                Some(24),
                ArmorCategories::Cloth,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::Kevlar => {
            requirements.insert(CombatAttributes::Intelligence, 10);
            ArmorGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(30, 40),
                Some(30),
                ArmorCategories::Cloth,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::LeatherArmor => {
            requirements.insert(CombatAttributes::Dexterity, 3);
            ArmorGenerationTemplate::new(
                Range::new(1, 5),
                Range::new(15, 22),
                Some(8),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::HardLeatherArmor => {
            requirements.insert(CombatAttributes::Dexterity, 5);
            ArmorGenerationTemplate::new(
                Range::new(3, 7),
                Range::new(25, 35),
                Some(15),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::StuddedLeatherArmor => {
            requirements.insert(CombatAttributes::Dexterity, 7);
            ArmorGenerationTemplate::new(
                Range::new(4, 10),
                Range::new(30, 45),
                Some(24),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::DemonsaurArmor => {
            requirements.insert(CombatAttributes::Dexterity, 15);
            ArmorGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(55, 65),
                Some(40),
                ArmorCategories::Leather,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::RingMail => {
            requirements.insert(CombatAttributes::Strength, 3);
            requirements.insert(CombatAttributes::Dexterity, 3);
            ArmorGenerationTemplate::new(
                Range::new(2, 5),
                Range::new(20, 24),
                Some(12),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::ChainMail => {
            requirements.insert(CombatAttributes::Strength, 3);
            requirements.insert(CombatAttributes::Dexterity, 3);
            requirements.insert(CombatAttributes::Intelligence, 3);
            ArmorGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(28, 36),
                Some(18),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::ScaleMail => {
            requirements.insert(CombatAttributes::Dexterity, 5);
            requirements.insert(CombatAttributes::Intelligence, 5);
            ArmorGenerationTemplate::new(
                Range::new(4, 7),
                Range::new(34, 45),
                Some(24),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::SplintMail => {
            requirements.insert(CombatAttributes::Strength, 7);
            requirements.insert(CombatAttributes::Intelligence, 7);
            ArmorGenerationTemplate::new(
                Range::new(5, 9),
                Range::new(48, 60),
                Some(30),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::OhmushellMail => {
            requirements.insert(CombatAttributes::Strength, 10);
            requirements.insert(CombatAttributes::Dexterity, 10);
            requirements.insert(CombatAttributes::Intelligence, 10);
            ArmorGenerationTemplate::new(
                Range::new(65, 80),
                Range::new(1, 1),
                Some(50),
                ArmorCategories::Mail,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::BreastPlate => {
            requirements.insert(CombatAttributes::Strength, 5);
            ArmorGenerationTemplate::new(
                Range::new(2, 4),
                Range::new(30, 40),
                Some(18),
                ArmorCategories::Plate,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::FieldPlate => {
            requirements.insert(CombatAttributes::Strength, 7);
            ArmorGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(40, 45),
                Some(24),
                ArmorCategories::Plate,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::GothicPlate => {
            requirements.insert(CombatAttributes::Strength, 12);
            ArmorGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(50, 60),
                Some(30),
                ArmorCategories::Plate,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::FullPlate => {
            requirements.insert(CombatAttributes::Strength, 15);
            ArmorGenerationTemplate::new(
                Range::new(6, 8),
                Range::new(60, 75),
                Some(50),
                ArmorCategories::Plate,
                Some(requirements),
                None,
                None,
            )
        }
        BodyArmors::ShardPlate => {
            requirements.insert(CombatAttributes::Strength, 20);
            ArmorGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(80, 100),
                Some(80),
                ArmorCategories::Plate,
                Some(requirements),
                None,
                None,
            )
        }
    }
}

pub static BODY_ARMOR_GENERATION_TEMPLATES: Lazy<HashMap<BodyArmors, ArmorGenerationTemplate>> =
    Lazy::new(|| generate_templates(body_armor_template_from_base_item));

pub static BODY_ARMORS_BY_LEVEL: Lazy<HashMap<u8, Vec<BodyArmors>>> = Lazy::new(|| {
    let items_and_level_ranges: Vec<(&BodyArmors, &Range<u8>)> = BODY_ARMOR_GENERATION_TEMPLATES
        .iter()
        .collect::<Vec<(&BodyArmors, &ArmorGenerationTemplate)>>()
        .iter()
        .map(|template| (template.0, &template.1.template_properties.level_range))
        .collect::<Vec<(&BodyArmors, &Range<u8>)>>();
    items_by_level(items_and_level_ranges)
});
