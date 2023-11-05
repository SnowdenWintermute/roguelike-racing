use super::{headgear_generation_templates::HEADGEAR_GENERATION_TEMPLATES, HeadGears};
use crate::{
    app_consts::DEEPEST_FLOOR,
    items::{body_armor::ArmorGenerationTemplate, items_by_level::items_by_level},
    primatives::Range,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static HEADGEARS_BY_LEVEL: Lazy<HashMap<u8, Vec<HeadGears>>> = Lazy::new(|| {
    let templates: Vec<(&HeadGears, &ArmorGenerationTemplate)> =
        HEADGEAR_GENERATION_TEMPLATES.iter().collect();
    items_by_level(templates)
});
