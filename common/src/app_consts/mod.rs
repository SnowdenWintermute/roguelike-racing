pub mod error_messages;

pub const CHARACTER_INVENTORY_DEFAULT_CAPACITY: u8 = 3;
pub const DEEPEST_FLOOR: u8 = 10;
pub const LEVEL_TO_REACH_FOR_ESCAPE: u8 = 6;
pub const MAX_PARTY_SIZE: u8 = 3;
pub const MAIN_CHAT_CHANNEL: &str = "chat-main";
pub const LOBBY_CHANNEL: &str = "lobby";

// EQUIPMENT
pub const TWO_HANDED_WEAPON_ATTRIBUTE_MULTIPLIER: f32 = 1.75;

// ATTRIBUTES
pub const DEX_TO_ACCURACY_RATIO: u16 = 2;
pub const INT_TO_FOCUS_RATIO: u16 = 2;
pub const INT_TO_MP_RATIO: u16 = 2;
pub const FOCUS_TO_CRIT_CHANCE_RATIO: f32 = 0.5;
pub const AGI_TO_EVASION_RATIO: u16 = 2;
pub const AGI_TO_SPEED_RATIO: u16 = 1;
pub const VIT_TO_HP_RATIO: u16 = 2;
pub const VIT_TO_PERCENT_PHYSICAL_DAMAGE_REDUCTION_RATIO: f32 = 0.75;
pub const OFF_HAND_ACCURACY_MODIFIER: u8 = 75;
pub const OFF_HAND_DAMAGE_MODIFIER: u8 = 60;
pub const TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER: u16 = 2;
pub const RESILIENCE_TO_PERCENT_MAGICAL_DAMAGE_REDUCTION_RATIO: f32 = 0.75;

// COMBAT
pub const BASE_CRIT_CHANCE: u16 = 5;
pub const MAX_CRIT_CHANCE: u16 = 95;
pub const BASE_CRIT_MULTIPLIER: f32 = 1.5;
pub const MULTI_TARGET_HP_CHANGE_BONUS: f32 = 0.15;
pub const MIN_HIT_CHANCE: i16 = 5;
