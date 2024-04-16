use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use yewdux::Dispatch;

pub fn game_full_update_handler(
    game_dispatch: Dispatch<GameStore>,
    // bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    update: Option<RoguelikeRacerGame>,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        store.game = update;

        // let mut character_home_location = HomeLocation(Transform::from_xyz(
        //     -COMBATANT_POSITION_SPACING_SIDE,
        //     0.0,
        //     -COMBATANT_POSITION_SPACING_BETWEEN_ROWS / 2.0,
        // ));
        // character_home_location.0.rotate_y(PI);

        // for character_id in cloned_character_positions {
        //     let species = CombatantSpecies::Humanoid;
        // transmitter
        //     .send(MessageFromYew::DespawnCombatantModel(character_id))
        //     .expect("to send message");

        // transmitter
        //     .send(MessageFromYew::SpawnCharacterWithHomeLocation(
        //         character_id,
        //         character_home_location.clone(),
        //         species,
        //     ))
        //     .expect("could not send event");
        // character_home_location.0.translation.x += COMBATANT_POSITION_SPACING_SIDE;
        // }
    });
    Ok(())
}
