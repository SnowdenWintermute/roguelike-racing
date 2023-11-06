use super::HeadGears;
use crate::{
    combatants::CombatAttributes,
    items::{
        affixes::{PrefixTypes, SuffixTypes},
        body_armor::{ArmorCategories, ArmorGenerationTemplate},
        item_generation_template_properties::ItemGenerationTemplateAffixModifiers,
        items_by_level::items_by_level,
    },
    primatives::Range,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub static HEADGEAR_GENERATION_TEMPLATES: Lazy<HashMap<HeadGears, ArmorGenerationTemplate>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        let headgears: Vec<HeadGears> = HeadGears::iter().collect();
        let mut i = 0;
        while i < headgears.len() {
            let headgear = headgears[i];
            let mut requirements: HashMap<CombatAttributes, u8> = HashMap::new();
            let template = match headgear {
                HeadGears::Cap => ArmorGenerationTemplate::new(
                    Range::new(1, 3),
                    Range::new(1, 3),
                    10,
                    ArmorCategories::Cloth,
                    requirements,
                    None,
                ),
                HeadGears::Bandana => ArmorGenerationTemplate::new(
                    Range::new(2, 4),
                    Range::new(2, 4),
                    12,
                    ArmorCategories::Cloth,
                    requirements,
                    None,
                ),
                HeadGears::PaddedCap => ArmorGenerationTemplate::new(
                    Range::new(3, 6),
                    Range::new(3, 6),
                    20,
                    ArmorCategories::Cloth,
                    requirements,
                    None,
                ),
                HeadGears::Ribbon => ArmorGenerationTemplate::new(
                    Range::new(5, 10),
                    Range::new(1, 1),
                    20,
                    ArmorCategories::Cloth,
                    requirements,
                    Some(ItemGenerationTemplateAffixModifiers::new(
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
                ),
                HeadGears::WizardHat => ArmorGenerationTemplate::new(
                    Range::new(8, 10),
                    Range::new(6, 14),
                    40,
                    ArmorCategories::Cloth,
                    requirements,
                    None,
                ),
                HeadGears::Eyepatch => ArmorGenerationTemplate::new(
                    Range::new(1, 3),
                    Range::new(2, 5),
                    14,
                    ArmorCategories::Leather,
                    requirements,
                    None,
                ),
                HeadGears::LeatherHat => ArmorGenerationTemplate::new(
                    Range::new(2, 5),
                    Range::new(5, 8),
                    20,
                    ArmorCategories::Leather,
                    requirements,
                    None,
                ),
                HeadGears::LeatherHelm => ArmorGenerationTemplate::new(
                    Range::new(4, 8),
                    Range::new(9, 15),
                    35,
                    ArmorCategories::Leather,
                    requirements,
                    None,
                ),
                HeadGears::DemonsaurHelm => ArmorGenerationTemplate::new(
                    Range::new(9, 10),
                    Range::new(18, 24),
                    45,
                    ArmorCategories::Leather,
                    requirements,
                    None,
                ),
                HeadGears::Hairpin => ArmorGenerationTemplate::new(
                    Range::new(3, 4),
                    Range::new(2, 2),
                    20,
                    ArmorCategories::Mail,
                    requirements,
                    None,
                ),
                HeadGears::Skullcap => ArmorGenerationTemplate::new(
                    Range::new(3, 6),
                    Range::new(8, 16),
                    28,
                    ArmorCategories::Mail,
                    requirements,
                    None,
                ),
                HeadGears::Coif => ArmorGenerationTemplate::new(
                    Range::new(4, 8),
                    Range::new(20, 26),
                    36,
                    ArmorCategories::Mail,
                    requirements,
                    None,
                ),
                HeadGears::OhmushellMask => ArmorGenerationTemplate::new(
                    Range::new(7, 10),
                    Range::new(30, 38),
                    50,
                    ArmorCategories::Mail,
                    requirements,
                    None,
                ),
                HeadGears::Circlet => ArmorGenerationTemplate::new(
                    Range::new(2, 5),
                    Range::new(5, 10),
                    30,
                    ArmorCategories::Plate,
                    requirements,
                    None,
                ),
                HeadGears::Crown => ArmorGenerationTemplate::new(
                    Range::new(3, 7),
                    Range::new(10, 20),
                    35,
                    ArmorCategories::Plate,
                    requirements,
                    None,
                ),
                HeadGears::FullHelm => ArmorGenerationTemplate::new(
                    Range::new(5, 10),
                    Range::new(22, 30),
                    40,
                    ArmorCategories::Plate,
                    requirements,
                    None,
                ),
                HeadGears::GreatHelm => ArmorGenerationTemplate::new(
                    Range::new(9, 10),
                    Range::new(32, 40),
                    50,
                    ArmorCategories::Plate,
                    requirements,
                    None,
                ),
            };

            m.insert(headgear, template);
            i += 1;
        }
        m
    });

pub static HEADGEARS_BY_LEVEL: Lazy<HashMap<u8, Vec<HeadGears>>> = Lazy::new(|| {
    let templates: Vec<(&HeadGears, &ArmorGenerationTemplate)> =
        HEADGEAR_GENERATION_TEMPLATES.iter().collect();
    items_by_level(templates)
});
