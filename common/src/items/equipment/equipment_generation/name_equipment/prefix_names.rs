use crate::items::equipment::affixes::PrefixTypes;

pub fn get_prefix_name(prefix_type: &PrefixTypes, tier: &u8) -> &'static str {
    match prefix_type {
        PrefixTypes::Mp => match tier {
            1 => "Bluejay's",
            2 => "Cockatoo's",
            3 => "Owl's",
            4 => "Kea's",
            5 => "Raven's",
            _ => "hacked",
        },
        PrefixTypes::ArmorClass => match tier {
            1 => "Sturdy",
            2 => "Strong",
            3 => "Robust",
            4 => "Reinforced",
            5 => "Unyielding",
            _ => "hacked",
        },
        PrefixTypes::Accuracy => match tier {
            1 => "Steady",
            2 => "Stable",
            3 => "Sighted",
            4 => "Guided",
            5 => "Precient",
            _ => "hacked",
        },
        PrefixTypes::PercentDamage => match tier {
            1 => "Jagged",
            2 => "Deadly",
            3 => "Vicious",
            4 => "Brutal",
            5 => "Savage",
            _ => "hacked",
        },
        PrefixTypes::LifeSteal => match tier {
            1 => "Mosquito's",
            2 => "Tick's",
            3 => "Leech's",
            4 => "Bat's",
            5 => "Lamprey's",
            _ => "hacked",
        },
        PrefixTypes::Resilience => match tier {
            1 => "Spirited",
            2 => "Hardy",
            3 => "Tenacious",
            4 => "Stalwart",
            5 => "Resolute",
            _ => "hacked",
        },
        PrefixTypes::Evasion => match tier {
            1 => "Monkey's",
            2 => "Rabbit's",
            3 => "Squirrel's",
            4 => "Chipmonk's",
            5 => "Field Mouse's",
            _ => "hacked",
        },
        PrefixTypes::Obscurity => match tier {
            1 => "Cloudy",
            2 => "Mysterious",
            3 => "Esoteric",
            4 => "Cryptic",
            5 => "Arcane",
            _ => "hacked",
        },
        PrefixTypes::ArmorPenetration => match tier {
            1 => "Heavy",
            2 => "Dense",
            3 => "Solid",
            4 => "Puncturing",
            5 => "Penetrating",
            _ => "hacked",
        },
    }
}