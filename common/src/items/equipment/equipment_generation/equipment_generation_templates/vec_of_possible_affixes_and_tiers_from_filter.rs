pub fn vec_of_possible_affixes_and_tiers_from_filter<T>(all_affix_types: Vec<T>, get_tier_if_type_allowed: fn(affix_type:&T) -> Option<u8>) -> Vec<(T, u8)> 
where T: Clone
{
    let mut possible_affixes_and_tiers: Vec<(T, u8)> = vec![];
    let mut i = 0;
    while i < all_affix_types.len() {
        let affix_type = all_affix_types[i].clone();
        let max_tier_option = get_tier_if_type_allowed(&affix_type);
        if let Some(max_tier) = max_tier_option {
            possible_affixes_and_tiers.push((affix_type, max_tier))
        }
        i += 1;
    }

    possible_affixes_and_tiers
}
