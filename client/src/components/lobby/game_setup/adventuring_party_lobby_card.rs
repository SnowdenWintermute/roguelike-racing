use std::{collections::HashMap, hash::Hash};

use common::{
    adventuring_party::AdventuringParty, character::Character,
    packets::client_to_server::PlayerInputs,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::{
        common_components::atoms::button_basic::ButtonBasic,
        lobby::game_setup::character_creation_menu::CharacterCreationMenu,
        websocket_manager::send_client_input::send_client_input,
    },
    store::{game_store::GameStore, websocket_store::WebsocketStore},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub party: AdventuringParty,
}

#[function_component(AdventuringPartyLobbyCard)]
pub fn adventuring_party_lobby_card(props: &Props) -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (game_state, _) = use_store::<GameStore>();

    let leave_party = Callback::from(move |_| {
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::LeaveAdventuringParty,
        )
    });

    let mut characters_by_username: HashMap<String, Vec<Character>> = HashMap::new();
    for username in &props.party.player_usernames {
        let mut characters: Vec<Character> = Vec::new();
        for character in &props.party.characters {
            if username == &character.1.name_of_controlling_user {
                characters.push(character.1.clone());
            }
            characters_by_username.insert(username.clone(), characters.clone());
        }
    }

    html!(
        <div class="p-3 border border-slate-400 w-full mb-2">
            <h3 class="mb-2">{ "Party: "  }{props.party.name.clone()}</h3>
            if let Some(current_party_id) = game_state.current_party_id {
                if current_party_id == props.party.id {
                    <div class="mb-2">
                        <ButtonBasic onclick={leave_party} >{ "Leave Party" }</ButtonBasic>
                    </div>
                    <CharacterCreationMenu />
                    }
            }
            {characters_by_username.iter().map(|username_with_characters|
                html!{
                    <div>
                    {username_with_characters.0}{": "}
                    if username_with_characters.1.len() < 1 {
                        {"No characters yet..."}
                    } else {
                        {username_with_characters.1.iter().map(|character|
                            html!(<div>{format!("{}", character.combatant_properties.combatant_class)}</div>)
                         ).collect::<Html>()}
                    }
                    </div>
                }).collect::<Html>()}
            <div>
            </div>
        </div>
    )
}
