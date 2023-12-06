use crate::store::game_store::GameStore;
use common::primatives::NextOrPrevious;
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_cycle_targets(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    direction: NextOrPrevious,
) {
    game_dispatch.reduce_mut(|game_store| {
        let focused_character: _ = game_store
            .get_mut_character(game_store.focused_character_id)
            .expect("to have a valid focused character");
        let target_ids = &focused_character
            .combatant_properties
            .ability_target_ids
            .as_ref()
            .expect("an ability to be selected when cycling targets");
        let selected_ability = &focused_character
            .combatant_properties
            .abilities
            .get(
                focused_character
                    .combatant_properties
                    .selected_ability_name
                    .as_ref()
                    .expect("an ability to be selected when cycling targets"),
            )
            .expect("an ability to be selected when cycling targets");
        if target_ids.len() == 1 {
            let target_id = target_ids[0];
            match selected_ability.valid_targets {
                common::combatants::abilities::ValidTargets::Opponent => todo!(),
                common::combatants::abilities::ValidTargets::Friendly => todo!(),
                common::combatants::abilities::ValidTargets::Any => todo!(),
                common::combatants::abilities::ValidTargets::User => (),
            }
        }
        // determine if selecting a monster or character
        // if selecting multiple targets
        //   if selecting monsters, select characters else reverse dependent on ValidTargets
        // find the next or previous index
        // if it is out of bounds, loop to the next

        // send_client_input(
        //     &websocket_state.websocket,
        //     PlayerInputs::ChangeTargetIds(ClientSelectAbilityPacket {
        //         character_id: focused_character.entity_properties.id,
        //         ability_name_option: None,
        //     }),
        // )
    });
}
