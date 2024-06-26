use crate::yew_app::components::common_components::atoms::divider::Divider;
use crate::yew_app::components::common_components::molocules::text_submit::TextSubmit;
use crate::yew_app::components::lobby::game_setup::adventuring_party_lobby_card::AdventuringPartyLobbyCard;
use crate::yew_app::components::websocket_manager::send_client_input::send_client_input;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::errors::AppError;
use common::packets::client_to_server::PlayerInputs;
use std::ops::Deref;
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
            <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400 pointer-events-auto" id="game_list">
                <div class="mb-2" >
                    <h2 class="text-lg mb-2" >{"Game: "} {game.name}</h2>
                    <p class="text-ffxipink mb-2" >{"Create a party and some characters. For the best chance of survival, a party of three is suggested."}
                    </p>
                    <p class="text-ffxipink mb-2" >{"Invite friends to party together or compete in a race, or you may control all the characters in a party and play solo."}</p>
                    <Divider />
                    <TextSubmit
                        input_name={"new adventuring party name"}
                        input_placeholder={"New party name..."}
                        button_title={"Create Party"}
                        submit_disabled={false}
                        submit_handler_callback={create_party}
                    />
                </div>
                <div>
                    <h3>{ "Players not yet in a party:" }</h3>
                    <ul class="list-none">
                        {game.players.iter().map(|player|
                            html!{
                                if player.1.party_id.is_none() {
                                    <li>{player.1.username.clone()}</li>
                                }
                            }).collect::<Html>()}
                    </ul>
                </div>
                <div>
                    <h3 class="mb-2">{ "Adventuring Parties" }</h3>
                    {game.adventuring_parties.iter().map(|party|
                        html!{<AdventuringPartyLobbyCard party={party.1.clone()} />}).collect::<Html>()}
                </div>
            </section>
        ),
        Err(_err) => html! {
            <h1>
                {"no game found"}
            </h1>
        },
    }
}
