use std::collections::HashMap;

use crate::{app_consts::DEEPEST_FLOOR, primatives::Range};

use super::body_armor::ItemGenerationTemplate;

// pub fn items_by_level<T>(item_and_level_ranges: Vec<(&T, Range<u8>)>) -> HashMap<u8, Vec<T>>
// where
//     T: Clone,
// {
//     let mut m = HashMap::new();
//     let mut i = 1;
//     while i <= DEEPEST_FLOOR {
//         let mut v = Vec::new();
//         let mut j = 1;
//         while j < item_and_level_ranges.len() {
//             let curr = &item_and_level_ranges[j];
//             let range = &curr.1;
//             if i >= range.min && i <= range.max {
//                 let item = curr.0.clone();
//                 v.push(item)
//             }
//             j += 1;
//         }
//         m.insert(i, v);
//         i += 1;
//     }
//     m
// }

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
