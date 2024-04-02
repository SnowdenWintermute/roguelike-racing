use crate::yew_app::store::game_store::get_active_combatant;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_player;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(CharacterAutofocusManager)]
pub fn character_autofocus_manager() -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let prev_active_combatant_id_option: UseStateHandle<Option<u32>> = use_state(|| None);
    let focused_character_id = game_state.focused_character_id;
    let active_combatant_result = get_active_combatant(&game_state);
    let active_combatant_id_option = match active_combatant_result {
        Ok(combatant_option) => match combatant_option {
            Some((entity_properties, _)) => Some(entity_properties.id),
            None => None,
        },
        Err(_) => None,
    };
    let current_battle_id = game_state.current_battle_id;
    let prev_current_battle_id_state = use_state(|| None);

    use_effect_with(
        (
            focused_character_id,
            active_combatant_id_option,
            current_battle_id,
        ),
        move |_| {
            game_dispatch.reduce_mut(|store| match *prev_active_combatant_id_option {
                // if focusing active character and their turn ends, focus next active character
                Some(prev_active_combatant_id) => match active_combatant_id_option {
                    Some(new_active_combatant_id) => {
                        let party_result = game_state.get_current_party();
                        if let Ok(party) = party_result {
                            if party.character_positions.contains(&new_active_combatant_id) {
                                if focused_character_id == prev_active_combatant_id
                                    && !game_state.viewing_inventory
                                {
                                    store.focused_character_id = new_active_combatant_id
                                } else if !party
                                    .character_positions
                                    .contains(&prev_active_combatant_id)
                                    && !game_state.viewing_inventory
                                {
                                    store.focused_character_id = new_active_combatant_id
                                }
                            }
                        }
                    }
                    None => (),
                },
                None => (),
            });
            // if battle ended, focus first owned character
            let _ = (|| -> Result<(), AppError> {
                let party = game_state.get_current_party()?;
                let character_positions = party.character_positions.clone();
                if current_battle_id.is_none()
                    && prev_current_battle_id_state.is_some()
                    && !game_state.viewing_inventory
                {
                    let username = &lobby_state.username;
                    let game = game_state.get_current_game()?;
                    let player = get_player(game, &username)?;
                    for character_id in character_positions {
                        let player_owned_ids =
                            player.character_ids.clone().ok_or_else(|| AppError {
                                error_type: common::errors::AppErrorTypes::ClientError,
                                message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
                            })?;
                        if player_owned_ids.get(&character_id).is_some() {
                            game_dispatch
                                .reduce_mut(|store| store.focused_character_id = character_id);
                            break;
                        }
                    }
                } else {
                    game_dispatch.reduce_mut(|store| {
                        if let Some(active_combatant_id) = active_combatant_id_option {
                            if !store.viewing_inventory
                                && character_positions.contains(&active_combatant_id)
                            {
                                if store.current_battle_id != *prev_current_battle_id_state {
                                    store.focused_character_id = active_combatant_id
                                }
                            }
                        }
                    })
                }
                Ok(())
            })();
            prev_active_combatant_id_option.set(active_combatant_id_option);
            prev_current_battle_id_state.set(current_battle_id)
        },
    );

    html!()
}
