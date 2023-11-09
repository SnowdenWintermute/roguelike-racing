use crate::items::equipment::affixes::SuffixTypes;

pub fn get_suffix_name(suffix_type: &SuffixTypes, tier: &u8) -> &'static str {
    match suffix_type {
        SuffixTypes::Strength => match tier {
            1 => "Strength",
            2 => "Might",
            3 => "Power",
            4 => "Giants",
            5 => "Titans",
            _ => "hacked",
        },
        SuffixTypes::Intelligence => match tier {
            1 => "Intelligence",
            2 => "the Mind",
            3 => "Brilliance",
            4 => "Sorcery",
            5 => "Wizardry",
            _ => "hacked",
        },
        SuffixTypes::Dexterity => match tier {
            1 => "Dexterity",
            2 => "Proficiancy",
            3 => "Finesse",
            4 => "Mastery",
            5 => "Perfection",
            _ => "hacked",
        },
        SuffixTypes::Vitality => match tier {
            1 => "of Vitality",
            2 => "of Zest",
            3 => "of Vim",
            4 => "of Vigor",
            5 => "of Life",
            _ => "hacked",
        },
        SuffixTypes::AllBase => match tier {
            1 => "the Sky",
            2 => "the Moon",
            3 => "the Stars",
            4 => "the Heavens",
            5 => "the Zodiac",
            _ => "hacked",
        },
        SuffixTypes::Hp => match tier {
            1 => "the Fox",
            2 => "the Wolf",
            3 => "the Lion",
            4 => "the Bear",
            5 => "the Whale",
            _ => "hacked",
        },
        SuffixTypes::Focus => match tier {
            1 => "Attention",
            2 => "Concentration",
            3 => "Unveiling",
            4 => "Discovery",
            5 => "Revelation",
            _ => "hacked",
        },
        SuffixTypes::Damage => match tier {
            1 => "Harm",
            2 => "Ruin",
            3 => "Destruction",
            4 => "Devestation",
            5 => "Annihilation",
            _ => "hacked",
        },
        SuffixTypes::Durability => match tier {
            1 => "Sturdiness",
            2 => "Structure",
            3 => "the Ages",
            4 => "the Eons",
            5 => "Eternity",
            _ => "hacked",
        },
    }
}
