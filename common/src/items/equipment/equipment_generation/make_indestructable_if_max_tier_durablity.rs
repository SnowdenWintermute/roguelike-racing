use crate::{
    items::equipment::affixes::{Affix, SuffixTypes},
    primatives::MaxAndCurrent,
};

pub fn make_indestructable_if_max_tier_durability(
    affixes: &Vec<Affix>,
    durability: &mut Option<MaxAndCurrent<u8>>,
) {
    for affix in affixes {
        match affix {
            crate::items::equipment::affixes::Affix::Suffix(affix_type, tier) => match affix_type {
                SuffixTypes::Durability => {
                    if tier == &5 {
                        *durability = None
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
