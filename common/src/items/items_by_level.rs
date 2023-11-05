use super::item_generation_template_properties::ItemGenerationTemplate;
use crate::{app_consts::DEEPEST_FLOOR, primatives::Range};
use std::collections::HashMap;

pub fn items_by_level<T, U>(templates: Vec<(&T, &U)>) -> HashMap<u8, Vec<T>>
where
    T: Clone,
    U: ItemGenerationTemplate,
{
    let mut m = HashMap::new();
    let mut i = 1;
    while i <= DEEPEST_FLOOR {
        let mut v = Vec::new();
        let mut j = 1;
        while j < templates.len() {
            let curr = &templates[j];
            let range = curr.1.get_level_range();
            if i >= range.min && i <= range.max {
                let item = curr.0.clone();
                v.push(item)
            }
            j += 1;
        }
        m.insert(i, v);
        i += 1;
    }
    m
}
