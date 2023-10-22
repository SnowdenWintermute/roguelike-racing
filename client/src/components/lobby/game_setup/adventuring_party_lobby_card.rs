use common::{adventuring_party::AdventuringParty, packets::client_to_server::PlayerInputs};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::{
        common_components::atoms::button_basic::ButtonBasic,
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

    html!(
        <div class="p-3 border border-slate-400 w-full mb-2">
            <h3 class="mb-2">{ "Party: "  }{props.party.name.clone()}</h3>
            if let Some(current_party_id) = game_state.current_party_id {
                if current_party_id == props.party.id {
                        <ButtonBasic onclick={leave_party} >{ "Leave Party" }</ButtonBasic>
                    }
            }
            <div>
            </div>
            // <div>
            //     <Show when=move || is_own_party fallback=|| view! { <div></div> }>
            //         <CharacterCreationMenu />
            //     </Show>
            // </div>
            <div>
            //     <For
            //         each=move || character_memos().clone()
            //         key={move |character| (character().combatant_properties.combatant_class.clone()
            //         ,character().entity_properties.name.clone())}
            //         children=|character| {
            //             view! {
            //                 <div class="">
            //                     <div class="mb-2">{}</div>
            //                     <div>"characters:"</div>
            //                     {move || character().entity_properties.name}
            //                         // <CombatantClassDisplay character=character() />
            //                 </div>
            //             }
            //         }
            //     />
            </div>
        </div>
    )
}
