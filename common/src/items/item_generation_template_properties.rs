use crate::combatants::CombatAttributes;
use crate::items::affixes::{PrefixTypes, SuffixTypes};
use crate::primatives::Range;
use std::collections::HashMap;

pub struct ItemGenerationTemplateAffixModifiers {
    pub prefix_exclusions: Option<Vec<PrefixTypes>>,
    pub suffix_exclusions: Option<Vec<PrefixTypes>>,
    pub prefix_tier_overrides: Option<Vec<(PrefixTypes, u8)>>,
    pub suffix_tier_overrides: Option<Vec<(SuffixTypes, u8)>>,
}

impl ItemGenerationTemplateAffixModifiers {
    pub fn new(
        prefix_exclusions: Option<Vec<PrefixTypes>>,
        suffix_exclusions: Option<Vec<PrefixTypes>>,
        prefix_tier_overrides: Option<Vec<(PrefixTypes, u8)>>,
        suffix_tier_overrides: Option<Vec<(SuffixTypes, u8)>>,
    ) -> ItemGenerationTemplateAffixModifiers {
        ItemGenerationTemplateAffixModifiers {
            prefix_exclusions,
            suffix_exclusions,
            prefix_tier_overrides,
            suffix_tier_overrides,
        }
    }
}

pub struct ItemGenerationTemplateProperties {
    pub level_range: Range<u8>,
    pub ac_range: Option<Range<u8>>,
    pub damage: Option<Range<u8>>,
    pub max_durability: u8,
    pub requirements: HashMap<CombatAttributes, u8>,
    pub affix_modifiers: Option<ItemGenerationTemplateAffixModifiers>,
}

pub trait ItemGenerationTemplate {
    fn get_level_range(&self) -> &Range<u8>;
}
