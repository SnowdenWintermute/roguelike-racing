use super::{
    body_armor_generation_templates::BODY_ARMOR_GENERATION_TEMPLATES, ArmorGenerationTemplate,
    BodyArmors,
};
use crate::{app_consts::DEEPEST_FLOOR, items::items_by_level::items_by_level};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static BODY_ARMORS_BY_LEVEL: Lazy<HashMap<u8, Vec<BodyArmors>>> = Lazy::new(|| {
    let templates: Vec<(&BodyArmors, &ArmorGenerationTemplate)> =
        BODY_ARMOR_GENERATION_TEMPLATES.iter().collect();
    items_by_level(templates)
});
