use crate::combatants::CombatAttributes;
use crate::items::equipment::affixes::{PrefixTypes, SuffixTypes};
use crate::items::equipment::EquipmentTraits;
use crate::primatives::Range;
use std::collections::HashMap;

pub struct EquipmentGenerationTemplateAffixModifiers {
    pub prefix_exclusions: Option<Vec<PrefixTypes>>,
    pub suffix_exclusions: Option<Vec<SuffixTypes>>,
    pub prefix_tier_overrides: Option<Vec<(PrefixTypes, u8)>>,
    pub suffix_tier_overrides: Option<Vec<(SuffixTypes, u8)>>,
}

pub struct RefEquipmentGenerationTemplateAffixModifiers<'a> {
    pub prefix_exclusions: &'a Option<Vec<PrefixTypes>>,
    pub suffix_exclusions: &'a Option<Vec<SuffixTypes>>,
    pub prefix_tier_overrides: &'a Option<Vec<(PrefixTypes, u8)>>,
    pub suffix_tier_overrides: &'a Option<Vec<(SuffixTypes, u8)>>,
}

impl EquipmentGenerationTemplateAffixModifiers {
    pub fn new(
        prefix_exclusions: Option<Vec<PrefixTypes>>,
        suffix_exclusions: Option<Vec<SuffixTypes>>,
        prefix_tier_overrides: Option<Vec<(PrefixTypes, u8)>>,
        suffix_tier_overrides: Option<Vec<(SuffixTypes, u8)>>,
    ) -> EquipmentGenerationTemplateAffixModifiers {
        EquipmentGenerationTemplateAffixModifiers {
            prefix_exclusions,
            suffix_exclusions,
            prefix_tier_overrides,
            suffix_tier_overrides,
        }
    }
}

pub struct EquipmentGenerationTemplateProperties {
    pub level_range: Range<u8>,
    pub max_durability: u8,
    pub requirements: HashMap<CombatAttributes, u8>,
    pub affix_modifiers: Option<EquipmentGenerationTemplateAffixModifiers>,
    pub traits: Option<Vec<EquipmentTraits>>,
}

impl EquipmentGenerationTemplateProperties {
    pub fn get_affix_modifers(&self) -> RefEquipmentGenerationTemplateAffixModifiers {
        let affix_modifiers = match &self.affix_modifiers {
            Some(overrides) => Some(overrides),
            None => None,
        };
        let prefix_tier_overrides = match &affix_modifiers {
            Some(overrides) => &overrides.prefix_tier_overrides,
            None => &None,
        };
        let suffix_tier_overrides = match &affix_modifiers {
            Some(overrides) => &overrides.suffix_tier_overrides,
            None => &None,
        };
        let prefix_exclusions = match &affix_modifiers {
            Some(exclusions) => &exclusions.prefix_exclusions,
            None => &None,
        };
        let suffix_exclusions = match &affix_modifiers {
            Some(exclusions) => &exclusions.suffix_exclusions,
            None => &None,
        };

        RefEquipmentGenerationTemplateAffixModifiers {
            prefix_tier_overrides,
            suffix_tier_overrides,
            prefix_exclusions,
            suffix_exclusions,
        }
    }
}

pub trait EquipmentGenerationTemplate {
    fn get_level_range(&self) -> &Range<u8>;
}
