use super::affixes::{self, Affix, PrefixTypes, SuffixTypes};
use rand::Rng;

pub fn generate_equipment_affixes(
    prefixes_and_max_tiers: Vec<(PrefixTypes, u8)>,
    suffixes_and_max_tiers: Vec<(SuffixTypes, u8)>,
    level: u8,
) -> Vec<Affix> {
    let mut affixes: Vec<Affix> = Vec::new();
    let max_tier_modifier: f32 = 1.0;
    let min_tier_modifier: f32 = 0.5;

    for prefix_and_max_tier in prefixes_and_max_tiers {
        let min_tier = prefix_and_max_tier.1 as f32 * min_tier_modifier;
        let max_tier = prefix_and_max_tier.1 as f32 * max_tier_modifier;
        let tier_as_float = rand::thread_rng().gen_range(min_tier..=max_tier);
        let tier = tier_as_float.round() as u8;
        affixes.push(Affix::Prefix(prefix_and_max_tier.0, tier))
    }

    for suffix_and_max_tier in suffixes_and_max_tiers {
        let min_tier = suffix_and_max_tier.1 as f32 * min_tier_modifier;
        let max_tier = suffix_and_max_tier.1 as f32 * max_tier_modifier;
        let tier_as_float = rand::thread_rng().gen_range(min_tier..=max_tier);
        let tier = tier_as_float.round() as u8;
        affixes.push(Affix::Suffix(suffix_and_max_tier.0, tier))
    }

    affixes
}
