use common::adventuring_party::AdventuringParty;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub party: AdventuringParty,
    pub client_party_id: u32,
}

#[function_component(AdventuringPartyLobbyCard)]
pub fn adventuring_party_lobby_card(props: &Props) -> Html {
    html!(
        <div class="p-3 border border-slate-400 w-full mb-2">
            <h3 class="mb-2">{ "Party: "  }{props.party.name.clone()}</h3>
            // <div>
            //     <Show when=move || is_own_party fallback=|| view! { <div></div> }>
            //         <ButtonBasic on:click=leave_party>"Leave Party"</ButtonBasic>
            //     </Show>
            // </div>
            // <div>
            //     <Show when=move || is_own_party fallback=|| view! { <div></div> }>
            //         <CharacterCreationMenu />
            //     </Show>
            // </div>
            // <div>
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

            // </div>
        </div>
    )
}
