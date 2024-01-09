use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::lobby::game_setup::character_creation_menu::CharacterCreationMenu;
use crate::components::lobby::game_setup::character_lobby_card::CharacterLobbyCard;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::websocket_store::WebsocketStore;
use common::adventuring_party::AdventuringParty;
use common::character::Character;
use common::errors::AppError;
use common::packets::client_to_server::PlayerInputs;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub party: AdventuringParty,
}

#[function_component(AdventuringPartyLobbyCard)]
pub fn adventuring_party_lobby_card(props: &Props) -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (game_state, _) = use_store::<GameStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let game = game_state.game.clone().ok_or(AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Displaying game setup but no game found".to_string(),
    });

    let leave_party = Callback::from(move |_| {
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::LeaveAdventuringParty,
        )
    });

    let (websocket_state, _) = use_store::<WebsocketStore>();
    let party_id = props.party.id;
    let join_party = Callback::from(move |_| {
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::JoinAdventuringParty(party_id),
        )
    });

    // let mut characters_by_username: HashMap<String, Vec<Character>> = HashMap::new();
    let mut characters_by_username: Vec<(String, Vec<Character>)> = vec![];
    for username in &props.party.player_usernames {
        let mut characters: Vec<Character> = Vec::new();
        for character in &props.party.characters {
            if username == &character.1.name_of_controlling_user {
                characters.push(character.1.clone());
            }
        }
        characters_by_username.push((username.clone(), characters.clone()));
    }
    characters_by_username.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    match game {
        Ok(game) => html!(
            <div class="p-3 border border-slate-400 w-full mb-2">
                <h3 class="mb-2">{ "Party: "  }{props.party.name.clone()}</h3>
                if let Some(current_party_id) = game_state.current_party_id {
                    if current_party_id == props.party.id {
                        <div class="mb-2">
                            <ButtonBasic onclick={leave_party} >{ "Leave Party" }</ButtonBasic>
                        </div>
                        <CharacterCreationMenu />
                        }
                } else {
                        <div class="mb-2">
                            <ButtonBasic onclick={join_party} >{ "Join Party" }</ButtonBasic>
                        </div>
                }
                {characters_by_username.iter().map(|username_with_characters|{
                    let is_ready = game.players_readied.contains(&username_with_characters.0);
                    let ready_style = match is_ready  {
                            true => "bg-green-800",
                            false => ""
                        };

                    html!{
                        <div class={ready_style}>
                        {username_with_characters.0.clone()}{": "}
                        if username_with_characters.1.len() < 1 {
                            {"No characters yet..."}
                        } else {
                            {username_with_characters.1.iter().map(|character|
                                html!(
                                    <CharacterLobbyCard character={character.clone()}
                                      owned_by_self={username_with_characters.0 == lobby_state.username} />)
                             ).collect::<Html>()}
                        }
                        </div>
                    }}).collect::<Html>()}
                <div>
                </div>
            </div>
        ),
        Err(_) => html!(
        <div>
            {"No game found"}
        </div>
        ),
    }
}
