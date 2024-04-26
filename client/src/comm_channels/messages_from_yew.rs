use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::frontend_common::CharacterPartSelection;
use crate::frontend_common::CombatantSpecies;
use crate::yew_app::store::game_store::GameStore;
use common::combat::ActionResult;
use common::combat::CombatTurnResult;
use common::combatants::CombatantProperties;
use common::primatives::EntityId;
use std::collections::VecDeque;
use std::sync::Arc;
use yewdux::Dispatch;

// YEW MESSAGES
#[derive(Debug, Clone)]
pub enum MessageFromYew {
    SelectCharacterPart(CharacterPartSelection),
    SpawnCharacterWithHomeLocation(
        EntityId,
        HomeLocation,
        CombatantSpecies,
        CombatantProperties,
    ),
    DespawnCombatantModel(EntityId),
    NewTurnResults(VecDeque<CombatTurnResult>),
    NewRawActionResults(EntityId, Vec<ActionResult>),
    SetBevyRendering(bool),
    // GameDispatch(Arc<Dispatch<GameStore>>),
}
