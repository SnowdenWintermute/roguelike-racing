pub const UNRECOGNIZED_PACKET: &str = "An unrecognized packet was received";
pub const USER_NOT_FOUND: &str = "The game server couldn't find a user with the provided actor id";
pub const ACTOR_ID_NOT_FOUND: &str = "The actor id for that player is missing";
pub const GAME_NOT_FOUND: &str = "No game found by the provided name";
pub const MISSING_GAME_REFERENCE: &str = "Missing reference to current game";
pub const ALREADY_IN_GAME: &str = "You are already in a game";
pub const GAME_ALREADY_EXISTS: &str = "A game by that name already exists";
pub const GAME_HAS_STARTED: &str = "That game has already started";
// WEBSOCKET CHANNELS
pub const WEBSOCKET_CHANNEL_NOT_FOUND: &str = "No websocket channel found by that name";
pub const WEBSOCKET_NAMESPACE_NOT_FOUND: &str = "No websocket channel namespace found by that name";
// PARTIES
pub const PARTY_ALREADY_EXISTS: &str = "A party by that name already exists";
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
pub const NO_CHARACTERS_IN_PARTY: &str = "There are no characters in the party";
// ITEMS
pub const INVALID_ITEM_ID: &str = "No valid item with the provided id was found";
pub const ITEM_REQUIREMENTS_NOT_MET: &str =
    "You do not meet the requirements to equip or use that item";
pub const CANT_EQUIP_NON_EQUIPMENT: &str = "Only equipment items may be equipped";
pub const CANT_CONSUME_NON_CONSUMABLE_ITEM: &str = "Only consumable items may be consumed";
pub const INVALID_EQUIPMENT_EQUIPPED: &str =
    "An invalid item was found in a character's equippment slot";
pub const ITEM_NOT_YET_AVAILABLE: &str = "That item's data is still being sent to other clients";
pub const ITEM_ON_GROUND_ACKNOWLEDGEMENT_SENT_BEFORE_ITEM_EXISTED: &str =
"A client sent acknowledgement of receipt of an item on the ground before a registry was created for that item";
pub const TRIED_TO_DROP_ITEM_FROM_AN_EMPTY_SLOT: &str =
    "A client attempted to drop an item from an empty equipment slot";
pub const DROP_EQUIPPED_ITEM_SERVER_PACKET_MISMATCH: &str =
    "Received a packet telling a character to drop an equipped item but the item wasn't found";
pub const NO_BASE_EQUIPMENT_FOUND: &str = "No base equipment was provided";
pub const NO_CONSUMABLE_SELECTED: &str = "No consumable is selected";
pub const CONSUMABLE_NOT_FOUND: &str = "No consumable found";
pub const INVALID_EQUIPMENT_SLOT: &str = "An invalid equipment slot was provided";
// COMBATANTS
pub const ENEMY_COMBATANTS_NOT_FOUND: &str = "No enemy combatants found";
pub const ALLY_COMBATANTS_NOT_FOUND: &str = "No allied combatants found";
pub const COMBATANT_NOT_FOUND: &str = "No combatant was found by the provided ID";
// ATTRIBUTES
pub const NO_UNSPENT_ATTRIBUTE_POINTS: &str =
    "The selected combatant has no attribute points to spend";
// ABILITIES
pub const ABILITY_NOT_OWNED: &str = "That character doesn't have that ability";
pub const MISSING_ABILITY_REFERENCE: &str = "The character has no ability selected";
pub const ABILITY_HAS_NO_TARGETING_SCHEME: &str = "The selected ability has no targeting scheme";
pub const INSUFFICIENT_MP: &str = "Not enough mana";
// COMBAT ACTIONS
pub const NO_ACTION_SELECTED: &str = "No combat action is selected";
pub const INVALID_ACTION_TYPE: &str = "Invalid action type";
pub const INVALID_USABILITY_CONTEXT: &str = "The selected action can not be used in this context";
pub const MISSING_EXPECTED_ACTION_RESULT_DATA: &str = "An action result missing expected data";
pub const CANT_BE_USED_ON_DEAD_TARGET: &str =
    "The selected target is beyond the help of this ability";
pub const CANT_USE_ACTION_WHEN_DEAD: &str =
    "The selected combatant is unable to take actions due to being dead";
pub const MISSING_ACTION_HP_CHANGE_PROPERTIES: &str =
    "The provided combat action doesn't specify any Hp change properties";
pub const MISSING_ACTION_HP_CHANGE_BASE_VALUES: &str =
    "Tried to get the base Hp change values on an ability that doesn't specify them";
pub const ALREADY_FULL_HP: &str = "The target is already at full HP";
pub const ALREADY_FULL_MP: &str = "The target already has full mana";
// TARGETING
pub const NO_POSSIBLE_TARGETS_PROVIDED: &str =
    "No possible targets were found for the selected action";
pub const TRIED_TO_CYCLE_TARGETS_WHEN_NO_TARGETS: &str =
    "Client tried to cycle targets but they weren't targeting anything";
pub const NO_TARGETING_SCHEMES: &str = "The selected action has no targeting schemes";
pub const ONLY_ONE_TARGETING_SCHEME_AVAILABLE: &str =
    "The selected action has only one targeting scheme";
pub const INVALID_TARGETING_SCHEME: &str = "An invalid targeting scheme was selected";
pub const INVALID_TARGETS_SELECTED: &str = "Invalid targets provided";
pub const NO_VALID_TARGETS_FOUND: &str = "No valid targets exist for that action";
// BATTLES
pub const MISSING_BATTLE_REFERENCE: &str = "Missing a battle reference";
pub const BATTLE_NOT_FOUND: &str = "No battle was found by the provided ID";
pub const COMBATANT_BATTLE_MISMATCH: &str =
    "A combatant attempted to take action in a battle that they are not a part of";
pub const TURN_TRACKERS_EMPTY: &str = "The battle has no turn trackers in the turn order list";
pub const NOT_THIS_COMBATANTS_TURN: &str =
    "A combatant tried to use an ability when it was not their turn";
pub const NOT_ENOUGH_MOVEMENT: &str = "Tried to end a turn for a combatant without enough movement";
// EVENT PROCESSING
pub const COMBANTANT_MESH_MANAGER_NOT_FOUND: &str = "No mesh manager was found by that id";
pub const COMBANTANT_EVENT_MANAGER_NOT_FOUND: &str = "No event manager was found by that id";
pub const EVENT_MANAGER_MISSING_ACTION_RESULT: &str =
    "No action result found on this entity's event manager";
// EXPLORATION
pub const CANT_EXPLORE_WHEN_MONSTERS_ARE_PRESENT: &str =
    "Defeat the monsters before exploring the next room";
pub const CANT_DESCEND_IF_NO_STAIRS_ARE_PRESENT: &str =
    "You can't go down the stairs if there are no stairs to go down";
pub const MISSING_ROOM_TYPE_TO_GENERATE: &str =
    "Tried to generate a new room but the party's list of rooms to generate was empty";
pub const CLIENT_LIST_MISSING_ROOM_TYPE: &str = "The client's list of room types on the current floor didn't contain a room type at the specified index";
// YEW BEVY
pub const NO_YEW_TRANSMITTER_TO_BEVY: &str = "Unable to find a transmitter to the bevy app";
