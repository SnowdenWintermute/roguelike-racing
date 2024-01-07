pub const UNRECOGNIZED_PACKET: &str = "An unrecognized packet was received";
pub const USER_NOT_FOUND: &str = "The game server couldn't find a user with the provided actor id";
pub const GAME_NOT_FOUND: &str = "No game found by the provided name";
pub const MISSING_GAME_REFERENCE: &str = "Missing reference to current game";
pub const ALREADY_IN_GAME: &str = "You are already in a game";
pub const GAME_ALREADY_EXISTS: &str = "A game by that name already exists";
pub const GAME_HAS_STARTED: &str = "That game has already started";
// WEBSOCKET CHANNELS
pub const WEBSOCKET_CHANNEL_NOT_FOUND: &str = "No websocket channel found by that name";
pub const WEBSOCKET_NAMESPACE_NOT_FOUND: &str = "No websocket channel namespace found by that name";
// PARTIES
pub const PARTY_NOT_FOUND: &str = "No party found by that name";
pub const PARTY_FULL: &str = "The selected party is currently full";
pub const ALREADY_IN_PARTY: &str = "Leave your current party if you wish to create a new one";
pub const PLAYER_NOT_FOUND: &str = "No player was found by that username";
pub const MISSING_PARTY_REFERENCE: &str = "No party was found with the provided id";
pub const PARTY_CHARACTER_LIMIT_REACHED: &str =
    "Creating a character would exceed the maximum number allowed";
pub const CHARACTER_NOT_FOUND: &str = "No character found with the provided id";
pub const PLAYER_HAS_NO_CHARACTERS: &str = "No characters are owned by that player";
pub const CHARACTER_NOT_OWNED: &str =
    "A character with the provided id was not found in the player's list of owned characters";
pub const INVALID_ITEM_ID: &str =
    "No item with the provided id was found in the currently focused player's posession";
pub const ITEM_REQUIREMENTS_NOT_MET: &str =
    "You do not meet the requirements to equip or use that item";
pub const CANT_EQUIP_NON_EQUIPMENT: &str = "Only equipment items may be equipped";
pub const INVALID_EQUIPMENT_EQUIPPEND: &str =
    "An invalid item was found in a character's equippment sloty";
pub const CANT_EXPLORE_WHEN_MONSTERS_ARE_PRESENT: &str =
    "Defeat the monsters before exploring the next room";
pub const ENEMY_COMBATANTS_NOT_FOUND: &str = "No enemy combatants found";
pub const ALLY_COMBATANTS_NOT_FOUND: &str = "No allied combatants found";
pub const COMBATANT_NOT_FOUND: &str = "No combatant was found by the provided ID";
pub const NO_CHARACTERS_IN_PARTY: &str = "There are no characters in the party";
// ABILITIES
pub const ABILITY_NOT_OWNED: &str = "That character doesn't have that ability";
pub const NO_ABILITY_SELECTED: &str = "No ability is selected";
pub const MISSING_ABILITY_REFERENCE: &str = "The character has no ability selected";
pub const ABILITY_HAS_NO_TARGETING_SCHEME: &str = "The selected ability has no targeting scheme";
pub const INVALID_ABILITY_CONTEXT: &str = "The selected ability can not be used in this context";
// TARGETING
pub const NO_POSSIBLE_TARGETS_PROVIDED: &str =
    "No possible targets were found for the selected ability";
pub const TRIED_TO_CYCLE_TARGETS_WHEN_NO_TARGETS: &str =
    "Client tried to cycle targets but they weren't targeting anything";
pub const NO_TARGETING_SCHEMES: &str = "The selected ability has no targeting schemes";
pub const INVALID_TARGETING_SCHEME: &str = "An invalid targeting scheme was selected";
pub const INVALID_TARGETS_SELECTED: &str = "Invalid ability targets provided";
// BATTLES
pub const MISSING_BATTLE_REFERENCE: &str = "Missing a battle reference";
pub const BATTLE_NOT_FOUND: &str = "No battle was found by the provided ID";
pub const COMBATANT_BATTLE_MISMATCH: &str =
    "A combatant attempted to take action in a battle that they are not a part of";
pub const TURN_TRACKERS_EMPTY: &str = "The battle has no turn trackers in the turn order list";
pub const NOT_THIS_COMBATANTS_TURN: &str =
    "A combatant tried to use an ability when it was not their turn";
