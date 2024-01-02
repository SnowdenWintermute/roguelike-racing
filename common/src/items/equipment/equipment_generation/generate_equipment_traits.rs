use crate::items::equipment::affixes::Affix;
use crate::items::equipment::affixes::PrefixTypes;
use crate::items::equipment::affixes::SuffixTypes;
use crate::items::equipment::EquipmentTraits;

pub fn generate_equipment_traits(affixes: &Vec<Affix>) -> Option<Vec<EquipmentTraits>> {
    if affixes.len() < 1 {
        return None;
    }
    let mut traits_to_return = Vec::new();
    for affix in affixes {
        match affix {
            Affix::Prefix(prefix, tier) => match prefix {
                PrefixTypes::Mp => (),
                PrefixTypes::ArmorClass => (),
                PrefixTypes::Accuracy => (),
                PrefixTypes::PercentDamage => {
                    traits_to_return.push(EquipmentTraits::DamagePercentage(tier * 10))
                }
                PrefixTypes::LifeSteal => {
                    traits_to_return.push(EquipmentTraits::LifeStealPercentage(tier.clone()))
                }
                PrefixTypes::Resilience => (),
                PrefixTypes::Evasion => (),
                PrefixTypes::Obscurity => (),
                PrefixTypes::ArmorPenetration => (),
                PrefixTypes::Agility => todo!(),
            },
            Affix::Suffix(suffix, tier) => match suffix {
                SuffixTypes::Strength => (),
                SuffixTypes::Intelligence => (),
                SuffixTypes::Dexterity => (),
                SuffixTypes::Vitality => (),
                SuffixTypes::AllBase => (),
                SuffixTypes::Hp => (),
                SuffixTypes::Focus => (),
                SuffixTypes::Damage => (),
                SuffixTypes::Durability => {
                    if *tier != 5 {
                        // because tier 5 would make item indestructable, a durablity bonus would be
                        // meaningless
                        traits_to_return.push(EquipmentTraits::DurabilityBonus(tier * 10))
                    }
                }
            },
        }
    }

    Some(traits_to_return)
}
