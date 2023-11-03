use super::affixes::{self, Affix, PrefixTypes, SuffixTypes};
use crate::{combatants::CombatAttributes, primatives::Range};
use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashMap;

pub fn generate_equipment_attributes(affixes: Vec<Affix>) -> HashMap<CombatAttributes, u16> {
    let mut attributes: HashMap<CombatAttributes, u16> = HashMap::new();

    for affix in affixes {
        match affix {
            Affix::Prefix(prefix_type, tier) => {
                let tier = tier as f32;
                let mut min_value;
                let mut max_value;
                let mut attribute: CombatAttributes;
                match prefix_type {
                    PrefixTypes::Mp => {
                        attribute = CombatAttributes::Mp;
                        min_value = tier;
                        max_value = tier * 2.0;
                    }
                    PrefixTypes::ArmorClass => todo!(),
                    PrefixTypes::Accuracy => todo!(),
                    PrefixTypes::PercentDamage => todo!(),
                    PrefixTypes::LifeSteal => todo!(),
                    PrefixTypes::Resilience => todo!(),
                    PrefixTypes::Evasion => todo!(),
                    PrefixTypes::Obscurity => todo!(),
                    PrefixTypes::ArmorPenetration => todo!(),
                }
                let attribute_value = rand::thread_rng()
                    .gen_range(min_value.round() as u16..=max_value.round() as u16);
                attributes.insert(attribute, attribute_value);
            }
            Affix::Suffix(suffix_type, tier) => match suffix_type {
                SuffixTypes::Strength => todo!(),
                SuffixTypes::Intelligence => todo!(),
                SuffixTypes::Dexterity => todo!(),
                SuffixTypes::Vitality => todo!(),
                SuffixTypes::AllBase => todo!(),
                SuffixTypes::Hp => todo!(),
                SuffixTypes::Focus => todo!(),
                SuffixTypes::Damage => todo!(),
                SuffixTypes::Durability => todo!(),
            },
        }
    }

    attributes
}
