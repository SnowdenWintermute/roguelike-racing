use super::ArmorGenerationTemplate;
use crate::{
    combatants::CombatAttributes,
    items::{
        equipment::{armor_properties::ArmorCategories, body_armors::BodyArmors},
        items_by_level::items_by_level,
    },
    primatives::Range,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub static BODY_ARMOR_GENERATION_TEMPLATES: Lazy<HashMap<BodyArmors, ArmorGenerationTemplate>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        let armors: Vec<BodyArmors> = BodyArmors::iter().collect();
        let mut i = 0;
        while i < armors.len() {
            let armor = armors[i];
            let mut requirements = HashMap::new();
            let template = match armor {
                BodyArmors::Rags => ArmorGenerationTemplate::new(
                    Range::new(1, 3),
                    Range::new(2, 6),
                    6,
                    ArmorCategories::Cloth,
                    requirements,
                    None,
                    None,
                ),
                BodyArmors::Cape => ArmorGenerationTemplate::new(
                    Range::new(1, 4),
                    Range::new(5, 10),
                    12,
                    ArmorCategories::Cloth,
                    requirements,
                    None,
                    None,
                ),
                BodyArmors::Cloak => ArmorGenerationTemplate::new(
                    Range::new(3, 7),
                    Range::new(10, 14),
                    18,
                    ArmorCategories::Cloth,
                    requirements,
                    None,
                    None,
                ),
                BodyArmors::Robe => {
                    requirements.insert(CombatAttributes::Intelligence, 5);
                    ArmorGenerationTemplate::new(
                        Range::new(6, 9),
                        Range::new(18, 22),
                        24,
                        ArmorCategories::Cloth,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::Kevlar => {
                    requirements.insert(CombatAttributes::Intelligence, 10);
                    ArmorGenerationTemplate::new(
                        Range::new(8, 10),
                        Range::new(30, 40),
                        30,
                        ArmorCategories::Cloth,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::LeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 3);
                    ArmorGenerationTemplate::new(
                        Range::new(1, 5),
                        Range::new(15, 22),
                        8,
                        ArmorCategories::Leather,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::HardLeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 5);
                    ArmorGenerationTemplate::new(
                        Range::new(3, 7),
                        Range::new(25, 35),
                        15,
                        ArmorCategories::Leather,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::StuddedLeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 7);
                    ArmorGenerationTemplate::new(
                        Range::new(4, 10),
                        Range::new(30, 45),
                        24,
                        ArmorCategories::Leather,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::DemonsaurArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 15);
                    ArmorGenerationTemplate::new(
                        Range::new(8, 10),
                        Range::new(55, 65),
                        40,
                        ArmorCategories::Leather,
                        requirements,
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
                        12,
                        ArmorCategories::Mail,
                        requirements,
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
                        18,
                        ArmorCategories::Mail,
                        requirements,
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
                        24,
                        ArmorCategories::Mail,
                        requirements,
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
                        30,
                        ArmorCategories::Mail,
                        requirements,
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
                        50,
                        ArmorCategories::Mail,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::BreastPlate => {
                    requirements.insert(CombatAttributes::Strength, 5);
                    ArmorGenerationTemplate::new(
                        Range::new(2, 4),
                        Range::new(30, 40),
                        18,
                        ArmorCategories::Plate,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::FieldPlate => {
                    requirements.insert(CombatAttributes::Strength, 7);
                    ArmorGenerationTemplate::new(
                        Range::new(3, 6),
                        Range::new(40, 45),
                        24,
                        ArmorCategories::Plate,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::GothicPlate => {
                    requirements.insert(CombatAttributes::Strength, 12);
                    ArmorGenerationTemplate::new(
                        Range::new(5, 8),
                        Range::new(50, 60),
                        30,
                        ArmorCategories::Plate,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::FullPlate => {
                    requirements.insert(CombatAttributes::Strength, 15);
                    ArmorGenerationTemplate::new(
                        Range::new(6, 8),
                        Range::new(60, 75),
                        50,
                        ArmorCategories::Plate,
                        requirements,
                        None,
                        None,
                    )
                }
                BodyArmors::ShardPlate => {
                    requirements.insert(CombatAttributes::Strength, 20);
                    ArmorGenerationTemplate::new(
                        Range::new(8, 10),
                        Range::new(80, 100),
                        80,
                        ArmorCategories::Plate,
                        requirements,
                        None,
                        None,
                    )
                }
            };

            m.insert(armor, template);
            i += 1;
        }
        m
    });

pub static BODY_ARMORS_BY_LEVEL: Lazy<HashMap<u8, Vec<BodyArmors>>> = Lazy::new(|| {
    let items_and_level_ranges: Vec<(&BodyArmors, &Range<u8>)> = BODY_ARMOR_GENERATION_TEMPLATES
        .iter()
        .collect::<Vec<(&BodyArmors, &ArmorGenerationTemplate)>>()
        .iter()
        .map(|template| (template.0, &template.1.template_properties.level_range))
        .collect::<Vec<(&BodyArmors, &Range<u8>)>>();
    items_by_level(items_and_level_ranges)
});
