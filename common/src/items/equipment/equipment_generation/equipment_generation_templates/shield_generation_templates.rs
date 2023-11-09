use super::generate_templates::generate_templates;
use super::ShieldGenerationTemplate;
use crate::combatants::CombatAttributes;
use crate::items::equipment::affixes::SuffixTypes;
use crate::items::equipment::equipment_generation::equipment_generation_template_properties::EquipmentGenerationTemplateAffixModifiers;
use crate::items::equipment::shield_properties::ShieldSizes;
use crate::items::equipment::shields::Shields;
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn shield_template_from_base_item(
    item: &Shields,
    mut requirements: HashMap<CombatAttributes, u8>,
) -> ShieldGenerationTemplate {
    match item {
        Shields::MakeshiftBuckler => ShieldGenerationTemplate::new(
            Range::new(1, 3),
            Range::new(2, 6),
            ShieldSizes::Small,
            Some(6),
            requirements,
            None,
            None,
        ),
        Shields::WoodenKiteShield => {
            requirements.insert(CombatAttributes::Strength, 3);
            ShieldGenerationTemplate::new(
                Range::new(2, 4),
                Range::new(2, 6),
                ShieldSizes::Medium,
                Some(6),
                requirements,
                None,
                None,
            )
        }
        Shields::Buckler => ShieldGenerationTemplate::new(
            Range::new(3, 5),
            Range::new(2, 6),
            ShieldSizes::Small,
            Some(6),
            requirements,
            None,
            None,
        ),
        Shields::Pavise => ShieldGenerationTemplate::new(
            Range::new(4, 6),
            Range::new(2, 6),
            ShieldSizes::Large,
            Some(6),
            requirements,
            None,
            None,
        ),
        Shields::Aspis => ShieldGenerationTemplate::new(
            Range::new(5, 7),
            Range::new(2, 6),
            ShieldSizes::Medium,
            Some(6),
            requirements,
            None,
            None,
        ),
        Shields::LanternShield => ShieldGenerationTemplate::new(
            Range::new(5, 6),
            Range::new(2, 6),
            ShieldSizes::Small,
            Some(6),
            requirements,
            Some(EquipmentGenerationTemplateAffixModifiers {
                prefix_exclusions: None,
                suffix_exclusions: None,
                prefix_tier_overrides: None,
                suffix_tier_overrides: Some(vec![(SuffixTypes::Damage, 5)]),
            }),
            None,
        ),
        Shields::KiteShield => ShieldGenerationTemplate::new(
            Range::new(6, 8),
            Range::new(2, 6),
            ShieldSizes::Medium,
            Some(6),
            requirements,
            None,
            None,
        ),
        Shields::TowerShield => ShieldGenerationTemplate::new(
            Range::new(7, 10),
            Range::new(2, 6),
            ShieldSizes::Large,
            Some(6),
            requirements,
            None,
            None,
        ),
        Shields::AncientBuckler => ShieldGenerationTemplate::new(
            Range::new(8, 10),
            Range::new(2, 6),
            ShieldSizes::Small,
            Some(6),
            requirements,
            None,
            None,
        ),
        Shields::GothicShield => ShieldGenerationTemplate::new(
            Range::new(8, 10),
            Range::new(2, 6),
            ShieldSizes::Medium,
            Some(6),
            requirements,
            None,
            None,
        ),
    }
}

pub static SHIELD_GENERATION_TEMPLATES: Lazy<HashMap<Shields, ShieldGenerationTemplate>> =
    Lazy::new(|| generate_templates(shield_template_from_base_item));

pub static SHIELDS_BY_LEVEL: Lazy<HashMap<u8, Vec<Shields>>> = Lazy::new(|| {
    let items_and_level_ranges: Vec<(&Shields, &Range<u8>)> = SHIELD_GENERATION_TEMPLATES
        .iter()
        .collect::<Vec<(&Shields, &ShieldGenerationTemplate)>>()
        .iter()
        .map(|template| (template.0, &template.1.template_properties.level_range))
        .collect::<Vec<(&Shields, &Range<u8>)>>();
    items_by_level(items_and_level_ranges)
});
