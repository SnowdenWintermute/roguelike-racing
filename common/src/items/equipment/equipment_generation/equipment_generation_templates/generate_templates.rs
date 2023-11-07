use core::hash::Hash;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::combatants::CombatAttributes;

pub fn generate_templates<T, U>(
    template_from_base_item: fn(&T, HashMap<CombatAttributes, u8>) -> U,
) -> HashMap<T, U>
where
    T: IntoEnumIterator + Hash + std::cmp::Eq + Clone,
{
    let mut m = HashMap::new();
    let base_items: Vec<T> = T::iter().collect();
    let mut i = 0;
    while i < base_items.len() {
        let base_item = base_items[i].clone();
        let mut requirements: HashMap<CombatAttributes, u8> = HashMap::new();
        let template = template_from_base_item(&base_item, requirements);
        m.insert(base_item, template);
        i += 1;
    }
    m
}
