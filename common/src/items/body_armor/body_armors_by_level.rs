use super::{
    body_armor_generation_templates::BODY_ARMOR_GENERATION_TEMPLATES, ArmorGenerationTemplate,
    BodyArmors,
};
use crate::app_consts::DEEPEST_FLOOR;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static BODY_ARMORS_BY_LEVEL: Lazy<HashMap<u8, Vec<BodyArmors>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let templates: Vec<(&BodyArmors, &ArmorGenerationTemplate)> =
        BODY_ARMOR_GENERATION_TEMPLATES.iter().collect();
    let mut i = 1;
    while i <= DEEPEST_FLOOR {
        let mut v = Vec::new();
        let mut j = 1;
        while j < templates.len() {
            let template = templates[j];
            if i >= template.1.level_range.min && i <= template.1.level_range.max {
                let cloned_armor = template.0.clone();
                v.push(cloned_armor);
            }
            j += 1;
        }
        m.insert(i, v);
        i += 1;
    }
    m
});
