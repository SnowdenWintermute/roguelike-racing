use crate::components::common_components::molocules::text_submit::TextSubmit;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::websocket_store::WebsocketStore;
use common::combatants::combatant_classes::CombatantClass;
use common::packets::client_to_server::CharacterCreation;
use common::packets::client_to_server::PlayerInputs;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(CharacterCreationMenu)]
pub fn character_creation_menu() -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();

    let create_character = move |character_name: AttrValue| {
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateCharacter(CharacterCreation {
                character_name: character_name.deref().to_string(),
                combatant_class: CombatantClass::Warrior,
            }),
        );
    };

    html!(
        <div class="mb-2">
            <TextSubmit
                input_name={"character name"}
                input_placeholder={"Character name..."}
                button_title={"Create Character"}
                submit_disabled={false}
                submit_handler_callback={create_character}
            />
        </div>
    )
}
