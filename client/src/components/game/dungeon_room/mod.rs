use common::{character::Character, game::RoguelikeRacerGame};
pub mod combatant;
use yew::prelude::*;

use crate::components::game::dungeon_room::combatant::Combatant;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game: RoguelikeRacerGame,
    pub party_id: u32,
}

#[function_component(DungeonRoom)]
pub fn dungeon_room(props: &Props) -> Html {
    let game = &props.game;
    let party = game
        .adventuring_parties
        .get(&props.party_id)
        .expect("must have a party id");

    let mut characters = party
        .characters
        .clone()
        .into_iter()
        .collect::<Vec<(u32, Character)>>();
    characters.sort_by(move |a, b| {
        a.1.entity_properties
            .id
            .partial_cmp(&b.1.entity_properties.id)
            .unwrap()
    });

    html!(
        <section class="w-full h-[50%] border border-slate-400 bg-slate-700 mb-4 overflow-y-auto" >
            <div class="p-2 flex flex-col" >
                {characters.iter().map(|(_id, character)|
                    html!{<Combatant
                        entity_properties={character.entity_properties.clone()}
                        combatant_properties={character.combatant_properties.clone()} />}).collect::<Html>()}
            </div>
        </section>
    )
}
