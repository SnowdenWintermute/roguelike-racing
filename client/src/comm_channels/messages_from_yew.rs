use crate::bevy_app::modular_character_plugin::HomeLocation;
use common::combat::ActionResult;
use common::combat::CombatTurnResult;
use common::combatants::combatant_species::CombatantSpecies;
use common::combatants::CombatantProperties;
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use common::primatives::EntityId;
use common::primatives::EntityProperties;
use std::collections::VecDeque;

// YEW MESSAGES
#[derive(Debug, Clone)]
pub enum MessageFromYew {
    SpawnCharacterWithHomeLocation(
        EntityId,
        HomeLocation,
        CombatantSpecies,
        CombatantProperties,
        EntityProperties,
    ),
    DespawnCombatantModel(EntityId),
    EndGame,
    NewTurnResults(VecDeque<CombatTurnResult>),
    NewRawActionResults(EntityId, Vec<ActionResult>),
    SetBevyRendering(bool),
    CombatantPickedUpItem(EntityId, Item),
    CombatantDroppedItem(EntityId, EntityId),
    CombatantDroppedEquippedItem(EntityId, EquipmentSlots),
    CombatantUnequippedItem(EntityId, EquipmentSlots),
    CombatantEquippedItem(EntityId, EntityId, bool),
    // GameDispatch(Arc<Dispatch<GameStore>>),
}
