use crate::components::alerts::set_alert;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_focused_character;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::websocket_store::WebsocketStore;
use common::errors::AppError;
use common::game::getters::get_player;
use common::packets::client_to_server::CharacterAndCombatAction;
use common::packets::client_to_server::PlayerInputs;
use std::collections::HashSet;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u32,
    pub is_ally: bool,
}

#[function_component(FocusCharacterButton)]
pub fn focus_character_button(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let (_, alert_dispatch) = use_store::<AlertStore>();
    let focused_character_option = get_focused_character(&game_state);
    let is_focused_character = match focused_character_option {
        Ok(focused_character) => {
            if focused_character.entity_properties.id == props.id {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };

    let id = props.id;
    let handle_click = Callback::from(move |_| {
        let cloned_alert_dispatch = alert_dispatch.clone();
        let result = game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
            let character_switching_focus_away_from_id = store.focused_character_id;
            store.selected_item = None;
            store.focused_character_id = id;
            let game = store.get_current_game_mut()?;
            let username = lobby_state.username.clone();
            let player = get_player(game, &username)?;
            let player_character_ids_option = player.character_ids.clone();

            if player_character_ids_option
                .clone()
                .unwrap_or_else(|| HashSet::new())
                .get(&character_switching_focus_away_from_id)
                .is_some()
            {
                send_client_input(
                    &websocket_state.websocket,
                    PlayerInputs::SelectCombatAction(CharacterAndCombatAction {
                        character_id: character_switching_focus_away_from_id,
                        combat_action_option: None,
                    }),
                );
            }
            Ok(())
        });
        if let Err(error) = result {
            set_alert(cloned_alert_dispatch, error.message)
        }
    });
    let conditional_styles = if is_focused_character {
        "bg-green-700"
    } else {
        ""
    };

    html!(
        <div class={format!("border-l border-slate-400 w-10 max-w-10 min-w-10 {}", conditional_styles)} >
            if is_focused_character {
                <button class="flex items-center justify-center w-full h-full">
                {"X"}
                </button>
            } else if props.is_ally {
                <button class="flex items-center justify-center w-full h-full m-0" onclick={handle_click} >
                {"O"}
                </button>
            }
        </div>
    )
}
