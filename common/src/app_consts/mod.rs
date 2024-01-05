pub mod error_messages;

pub const CHARACTER_INVENTORY_DEFAULT_CAPACITY: u8 = 3;
pub const DEEPEST_FLOOR: u8 = 10;
pub const MAX_PARTY_SIZE: u8 = 3;
pub const MAIN_CHAT_CHANNEL: &str = "chat-main";
pub const LOBBY_CHANNEL: &str = "lobby";

// ATTRIBUTES
pub const DEX_TO_ACCURACY_RATIO: u16 = 2;
pub const INT_TO_FOCUS_RATIO: u16 = 2;
pub const AGI_TO_EVASION_RATIO: u16 = 2;
pub const AGI_TO_SPEED_RATIO: u16 = 1;
pub const OFF_HAND_ACCURACY_MODIFIER: f32 = 0.75;
pub const OFF_HAND_DAMAGE_MODIFIER: f32 = 0.60;
pub const TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER: u16 = 2;

// COMBAT
pub const TURN_TIME: i16 = 75;
