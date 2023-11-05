use super::{ArmorCategories, ArmorGenerationTemplate, BodyArmors};
use crate::combatants::CombatAttributes;
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
                BodyArmors::Rags => {
                    ArmorGenerationTemplate::new(1, 3, 6, ArmorCategories::Cloth, requirements)
                }
                BodyArmors::Cape => {
                    ArmorGenerationTemplate::new(1, 4, 12, ArmorCategories::Cloth, requirements)
                }
                BodyArmors::Cloak => {
                    ArmorGenerationTemplate::new(3, 7, 18, ArmorCategories::Cloth, requirements)
                }
                BodyArmors::Robe => {
                    requirements.insert(CombatAttributes::Intelligence, 5);
                    ArmorGenerationTemplate::new(6, 9, 24, ArmorCategories::Cloth, requirements)
                }
                BodyArmors::Kevlar => {
                    requirements.insert(CombatAttributes::Intelligence, 10);
                    ArmorGenerationTemplate::new(8, 10, 30, ArmorCategories::Cloth, requirements)
                }
                BodyArmors::LeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 3);
                    ArmorGenerationTemplate::new(1, 5, 8, ArmorCategories::Leather, requirements)
                }
                BodyArmors::HardLeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 5);
                    ArmorGenerationTemplate::new(3, 7, 15, ArmorCategories::Leather, requirements)
                }
                BodyArmors::StuddedLeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 7);
                    ArmorGenerationTemplate::new(4, 10, 24, ArmorCategories::Leather, requirements)
                }
                BodyArmors::DemonsaurArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 15);
                    ArmorGenerationTemplate::new(8, 10, 40, ArmorCategories::Leather, requirements)
                }
                BodyArmors::RingMail => {
                    requirements.insert(CombatAttributes::Strength, 3);
                    requirements.insert(CombatAttributes::Dexterity, 3);
                    ArmorGenerationTemplate::new(2, 5, 12, ArmorCategories::Mail, requirements)
                }
                BodyArmors::ChainMail => {
                    requirements.insert(CombatAttributes::Strength, 3);
                    requirements.insert(CombatAttributes::Dexterity, 3);
                    requirements.insert(CombatAttributes::Intelligence, 3);
                    ArmorGenerationTemplate::new(3, 6, 18, ArmorCategories::Mail, requirements)
                }
                BodyArmors::ScaleMail => {
                    requirements.insert(CombatAttributes::Dexterity, 5);
                    requirements.insert(CombatAttributes::Intelligence, 5);
                    ArmorGenerationTemplate::new(4, 7, 24, ArmorCategories::Mail, requirements)
                }
                BodyArmors::SplintMail => {
                    requirements.insert(CombatAttributes::Strength, 7);
                    requirements.insert(CombatAttributes::Intelligence, 7);
                    ArmorGenerationTemplate::new(5, 9, 30, ArmorCategories::Mail, requirements)
                }
                BodyArmors::OhmushellMail => {
                    requirements.insert(CombatAttributes::Strength, 10);
                    requirements.insert(CombatAttributes::Dexterity, 10);
                    requirements.insert(CombatAttributes::Intelligence, 10);
                    ArmorGenerationTemplate::new(8, 10, 50, ArmorCategories::Mail, requirements)
                }
                BodyArmors::BreastPlate => {
                    requirements.insert(CombatAttributes::Strength, 5);
                    ArmorGenerationTemplate::new(2, 4, 18, ArmorCategories::Plate, requirements)
                }
                BodyArmors::FieldPlate => {
                    requirements.insert(CombatAttributes::Strength, 7);
                    ArmorGenerationTemplate::new(3, 6, 24, ArmorCategories::Plate, requirements)
                }
                BodyArmors::GothicPlate => {
                    requirements.insert(CombatAttributes::Strength, 12);
                    ArmorGenerationTemplate::new(5, 8, 30, ArmorCategories::Plate, requirements)
                }
                BodyArmors::FullPlate => {
                    requirements.insert(CombatAttributes::Strength, 15);
                    ArmorGenerationTemplate::new(6, 8, 50, ArmorCategories::Plate, requirements)
                }
                BodyArmors::ShardPlate => {
                    requirements.insert(CombatAttributes::Strength, 20);
                    ArmorGenerationTemplate::new(8, 10, 80, ArmorCategories::Plate, requirements)
                }
            };

            m.insert(armor, template);
            i += 1;
        }
        m
    });
