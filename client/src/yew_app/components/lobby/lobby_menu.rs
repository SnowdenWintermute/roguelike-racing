use crate::yew_app::components::common_components::atoms::button_basic::ButtonBasic;
use crate::yew_app::components::common_components::atoms::text_input::TextInput;
use crate::yew_app::components::websocket_manager::send_client_input::send_client_input;
use crate::yew_app::store::lobby_store::LobbyStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::combatants::combatant_classes::CombatantClass;
use common::packets::client_to_server::CharacterCreation;
use common::packets::client_to_server::GameCreation;
use common::packets::client_to_server::PlayerInputs;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(LobbyMenu)]
pub fn lobby_menu() -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let game_name = use_state(|| AttrValue::from(""));

    let handle_change = {
        let game_name = game_name.clone();
        Callback::from(move |new_name| game_name.set(new_name))
    };

    let cloned_websocket_state = websocket_state.clone();
    let create_game = move |event: SubmitEvent| {
        event.prevent_default();
        send_client_input(
            &cloned_websocket_state.websocket,
            PlayerInputs::CreateGame(GameCreation {
                name: game_name.deref().clone().to_string(),
                password: None,
            }),
        );
    };

    let quick_create_game = move |_| {
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateGame(GameCreation {
                name: "".to_string(),
                password: None,
            }),
        );
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateAdventuringParty("".to_string()),
        );
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateCharacter(CharacterCreation {
                character_name: "".to_string(),
                combatant_class: CombatantClass::Mage,
            }),
        );
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateCharacter(CharacterCreation {
                character_name: "".to_string(),
                combatant_class: CombatantClass::Mage,
            }),
        );
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateCharacter(CharacterCreation {
                character_name: "".to_string(),
                combatant_class: CombatantClass::Mage,
            }),
        );

        send_client_input(&websocket_state.websocket, PlayerInputs::ToggleReady);
    };

    let (websocket_state, _) = use_store::<WebsocketStore>();
    let refresh_game_list = Callback::from(move |_| {
        send_client_input(&websocket_state.websocket, PlayerInputs::RequestGameList)
    });

    html!(
        <section class="w-full bg-slate-700 border border-slate-400 p-4 mb-4 flex justify-between pointer-events-auto">
            <div class="flex">
                <form class="flex mr-2" onsubmit={create_game}>
                    <TextInput name="game name" placeholder="Game name..." handle_change={handle_change} />
                    <ButtonBasic disabled=false extra_styles="border-l-0 text-yellow-400" button_type="submit" >
                        {"Create Game"}
                    </ButtonBasic>
                    <ButtonBasic disabled=false onclick={quick_create_game} extra_styles="ml-2" >
                        {"Quick Start"}
                    </ButtonBasic>
                </form>
                <ButtonBasic disabled=false button_type="button" onclick={refresh_game_list} >
                    {"Refresh List"}
                </ButtonBasic>
                // <svg class="fill-red-500 border border-red-500 flex h-10" >
                //     <use href="public/img/logo.svg#logo" />
                // </svg>
            </div>
            <div class="border border-slate-400 rounded-full h-10 w-10 flex justify-center items-center" >
                <span class="text-lg font-bold">
                    if lobby_state.username.clone().chars().collect::<Vec<char>>().len() > 0 {
                    {lobby_state.username.clone().chars().next().unwrap().to_uppercase()}}
                </span>
            </div>
        </section>
    )
}
