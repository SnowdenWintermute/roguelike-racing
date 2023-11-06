use crate::items::affixes::{PrefixTypes, SuffixTypes};
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

pub static ONE_HANDED_MELEE_WEAPONS_POSSIBLE_PREFIXES_AND_TIERS: Lazy<Vec<(PrefixTypes, u8)>> =
    Lazy::new(|| {
        let all_prefix_types: Vec<PrefixTypes> = PrefixTypes::iter().collect();
        let mut possible_prefixes_and_tiers: Vec<(PrefixTypes, u8)> = vec![];
        let mut i = 0;
        while i < all_prefix_types.len() {
            let prefix_type = all_prefix_types[i].clone();
            let max_tier_option = match prefix_type {
                PrefixTypes::Mp => None,
                PrefixTypes::ArmorClass => None,
                PrefixTypes::Accuracy => Some(5),
                PrefixTypes::PercentDamage => Some(5),
                PrefixTypes::LifeSteal => Some(5),
                PrefixTypes::Resilience => None,
                PrefixTypes::Evasion => None,
                PrefixTypes::Obscurity => None,
                PrefixTypes::ArmorPenetration => Some(5),
            };
            if let Some(max_tier) = max_tier_option {
                possible_prefixes_and_tiers.push((prefix_type, max_tier))
            }
            i += 1;
        }

        possible_prefixes_and_tiers
    });

pub static ONE_HANDED_MELEE_WEAPONS_POSSIBLE_SUFFIXES_AND_TIERS: Lazy<Vec<(SuffixTypes, u8)>> =
    Lazy::new(|| {
        let all_suffix_types: Vec<SuffixTypes> = SuffixTypes::iter().collect();
        let mut possible_suffixes_and_tiers: Vec<(SuffixTypes, u8)> = vec![];
        let mut i = 0;
        while i < all_suffix_types.len() {
            let suffix_type = all_suffix_types[i].clone();
            let max_tier_option = match suffix_type {
                SuffixTypes::Strength => Some(5),
                SuffixTypes::Intelligence => Some(5),
                SuffixTypes::Dexterity => Some(5),
                SuffixTypes::Vitality => Some(5),
                SuffixTypes::AllBase => Some(3),
                SuffixTypes::Hp => None,
                SuffixTypes::Focus => Some(5),
                SuffixTypes::Damage => Some(5),
                SuffixTypes::Durability => Some(5),
            };
            if let Some(max_tier) = max_tier_option {
                possible_suffixes_and_tiers.push((suffix_type, max_tier))
            }
            i += 1;
        }

        possible_suffixes_and_tiers
    });
