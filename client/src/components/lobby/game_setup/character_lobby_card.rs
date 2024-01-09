use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::websocket_store::WebsocketStore;
use common::character::Character;
use common::packets::client_to_server::PlayerInputs;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub character: Character,
    pub owned_by_self: bool,
}

#[function_component(CharacterLobbyCard)]
pub fn character_lobby_card(props: &Props) -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let character = &props.character;

    let id = character.entity_properties.id;
    let delete_character = Callback::from(move |_| {
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::DeleteCharacter(id),
        )
    });

    html!(
            <div>
                <div>
                    {&character.entity_properties.name}
                </div>
                <div>
                    <div>
                        {"Class: " }
                        {format!("{}", &character.combatant_properties.combatant_class)}
                    </div>
                    if props.owned_by_self {
                        <div>
                            <ButtonBasic onclick={ delete_character }>
                            {"Delete "}{&character.entity_properties.name}
                            </ButtonBasic>
                        </div>
                    }
                </div>
            </div>
    )
}
