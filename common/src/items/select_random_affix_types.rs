use rand::Rng;

pub fn select_random_affix_types<T: Clone>(
    possible_affix_types: &Vec<&T>,
    num_affixes: u8,
) -> Vec<T> {
    let mut affix_types_to_return = Vec::new();
    if num_affixes < 1 {
        return affix_types_to_return;
    }
    let mut remaining_affixes_possible = possible_affix_types.clone();
    for i in 0..num_affixes {
        // this shouldn't happen if we don't allow items with a higher number of prefixes than the
        // number of prefix types, but just in case we'll exit early
        if remaining_affixes_possible.len() < 1 {
            break;
        }
        let random_affix_index = rand::thread_rng().gen_range(0..remaining_affixes_possible.len());
        let affix_type = remaining_affixes_possible
            .remove(random_affix_index)
            .clone();
        affix_types_to_return.push(affix_type);
    }
    affix_types_to_return
}
