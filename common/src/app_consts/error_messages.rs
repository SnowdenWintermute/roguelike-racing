use super::MAX_PARTY_SIZE;

pub const USER_NOT_FOUND: &str = "The game server couldn't find a user with the provided actor id";
pub const GAME_NOT_FOUND: &str = "No game found by the provided name";
pub const MISSING_GAME_REFERENCE: &str = "Missing reference to current game";
pub const ALREADY_IN_GAME: &str = "You are already in a game";
pub const GAME_ALREADY_EXISTS: &str = "A game by that name already exists";
pub const GAME_HAS_STARTED: &str = "That game has already started";
pub const ROOM_NOT_FOUND: &str = "No room found by that name";
pub const PARTY_NOT_FOUND: &str = "No party found by that name";
pub const PARTY_FULL: &str = "The selected party is currently full";
pub const ALREADY_IN_PARTY: &str = "Leave your current party if you wish to create a new one";
pub const PLAYER_NOT_FOUND: &str = "No player was found by that username";
pub const MISSING_PARTY_REFERENCE: &str = "No party was found with the provided id";
pub const PARTY_CHARACTER_LIMIT_REACHED: &str =
    "Creating a character would exceed the maximum number allowed";
pub const CHARACTER_NOT_FOUND: &str = "No character found with the provided id";
