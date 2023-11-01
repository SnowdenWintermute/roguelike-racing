mod paper_doll;
// use crate::store::{game_store::GameStore, websocket_store::WebsocketStore};
use common::character::Character;
use yew::prelude::*;

use crate::components::game::character_sheet::paper_doll::PaperDoll;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub character: Character,
}

#[function_component(CharacterSheet)]
pub fn character_sheet(props: &Props) -> Html {
    // let (game_state, game_dispatch) = use_store::<GameStore>();
    // let (websocket_state, _) = use_store::<WebsocketStore>();
    let Props { character } = props;

    html!(
        <section class="p-2 flex-grow border border-slate-400 bg-slate-700 overflow-y-auto flex">
            <PaperDoll equipment={character.combatant_properties.equipment.clone()} />
            // {"Character sheet for id: "}{props.character.entity_properties.id}
        </section>
    )
}
