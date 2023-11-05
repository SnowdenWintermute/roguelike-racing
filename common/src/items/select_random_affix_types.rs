use rand::Rng;
use std::cmp::PartialEq;

pub fn select_random_affix_types<T: Clone + PartialEq>(
    possible_affix_types_with_tiers: &Vec<&(T, u8)>,
    num_affixes: u8,
    affix_overrides: &Option<Vec<(T, u8)>>,
) -> Vec<(T, u8)> {
    let mut affix_types_to_return = Vec::new();
    if num_affixes < 1 {
        return affix_types_to_return;
    }
    let mut remaining_affixes_with_tiers = possible_affix_types_with_tiers.clone();
    for i in 0..num_affixes {
        // this shouldn't happen if we don't allow items with a higher number of prefixes than the
        // number of prefix types, but just in case we'll exit early
        if remaining_affixes_with_tiers.len() < 1 {
            break;
        }
        let random_affix_index =
            rand::thread_rng().gen_range(0..remaining_affixes_with_tiers.len());
        let mut affix_type_and_max_tier = remaining_affixes_with_tiers
            .remove(random_affix_index)
            .clone();
        // here we can apply overrides to change the possible tier of affixes as specified in their
        // item generation templates
        if let Some(overrides) = affix_overrides {
            for (affix_to_override, tier) in overrides {
                if affix_to_override == &affix_type_and_max_tier.0 {
                    affix_type_and_max_tier.1 = *tier
                }
            }
        }
        //
        affix_types_to_return.push(affix_type_and_max_tier);
    }
    affix_types_to_return
}
