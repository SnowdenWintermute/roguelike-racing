use super::{affixes::Affix, equipment::EquipmentTraits};

pub fn generate_equipment_traits(affixes: &Vec<Affix>) -> Option<Vec<EquipmentTraits>> {
    if affixes.len() < 1 {
        return None;
    }
    let mut traits_to_return = Vec::new();
    for affix in affixes {
        match affix {
            Affix::Prefix(prefix, tier) => match prefix {
                super::affixes::PrefixTypes::Mp => (),
                super::affixes::PrefixTypes::ArmorClass => (),
                super::affixes::PrefixTypes::Accuracy => (),
                super::affixes::PrefixTypes::PercentDamage => {
                    traits_to_return.push(EquipmentTraits::DamagePercentage(tier * 10))
                }
                super::affixes::PrefixTypes::LifeSteal => {
                    traits_to_return.push(EquipmentTraits::LifeStealPercentage(tier.clone()))
                }
                super::affixes::PrefixTypes::Resilience => (),
                super::affixes::PrefixTypes::Evasion => (),
                super::affixes::PrefixTypes::Obscurity => (),
                super::affixes::PrefixTypes::ArmorPenetration => (),
            },
            Affix::Suffix(suffix, tier) => match suffix {
                super::affixes::SuffixTypes::Strength => (),
                super::affixes::SuffixTypes::Intelligence => (),
                super::affixes::SuffixTypes::Dexterity => (),
                super::affixes::SuffixTypes::Vitality => (),
                super::affixes::SuffixTypes::AllBase => (),
                super::affixes::SuffixTypes::Hp => (),
                super::affixes::SuffixTypes::Focus => (),
                super::affixes::SuffixTypes::Damage => (),
                super::affixes::SuffixTypes::Durability => (),
            },
        }
    }

    Some(traits_to_return)
}
