use std::ops::Deref;

use crate::{
    components::{
        common_components::molocules::text_submit::TextSubmit,
        lobby::game_setup::adventuring_party_lobby_card::AdventuringPartyLobbyCard,
        websocket_manager::send_client_input::send_client_input,
    },
    store::{game_store::GameStore, websocket_store::WebsocketStore},
};
use common::{
    errors::AppError,
    packets::{client_to_server::PlayerInputs, server_to_client::AdventuringPartyCreation},
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(CharacterAndPartySelection)]
pub fn character_and_party_selection() -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (game_state, _) = use_store::<GameStore>();
    let game = game_state.game.clone().ok_or(AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Displaying game setup but no game found".to_string(),
    });

    let create_party = move |party_name: AttrValue| {
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateAdventuringParty(party_name.deref().to_string()),
        );
    };

    match game {
        Ok(game) => html!(
            <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400" id="game_list">
                <h2>{"Game: "} {game.name}</h2>
                    <TextSubmit
                        input_name={"new adventuring party name"}
                        input_placeholder={"New party name..."}
                        button_title={"Create Party"}
                        submit_disabled={false}
                        submit_handler_callback={create_party}
                    />
                <div>
                    <h3>{ "Players not yet in a party:" }</h3>
                    <ul class="list-none">
                        // <For
                        //     each=players
                        //     key=|player| (player.1.username.clone(), player.1.party_id.is_none())
                        //     children=|player| player.1.party_id.is_none().then(move ||
                        //             view! { <li>{player.1.username.clone()}</li> }
                        //     )
                        // />
                    </ul>
                </div>
                <div>
                    <h3 class="mb-2">{ "Adventuring Parties" }</h3>
                    {game.adventuring_parties.iter().map(|party|
                        html!{<AdventuringPartyLobbyCard party={party.1.clone()} client_party_id={0} />}).collect::<Html>()}
                </div>
            </section>
        ),
        Err(err) => html! {
            <h1>
                {"no game found"}
            </h1>
        },
    }
}
