use common::character::Character;
use common::dungeon_rooms::DungeonRoomTypes;
use common::game::RoguelikeRacerGame;
pub mod combatant;
mod focus_character_button;
mod monster_lair;
mod players_ready_to_explore;
use crate::components::game::dungeon_room::combatant::Combatant;
use crate::components::game::dungeon_room::monster_lair::MonsterLair;
use crate::components::game::dungeon_room::players_ready_to_explore::PlayersReadyToExplore;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game: RoguelikeRacerGame,
    pub party_id: u32,
}

#[function_component(DungeonRoom)]
pub fn dungeon_room(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
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

    let conditional_styles = match game_state.viewing_inventory {
        true => "min-w-[350px] w-[350px] mr-4",
        false => "w-full",
    };

    html!(
        <section class={format!("h-full border border-slate-400 bg-slate-700 flex {}", conditional_styles)} >
            <div class="w-1/2 flex p-2" >
                <div class="flex flex-col mr-2" >
                    {characters.iter().map(|(_id, character)|
                        html!{<Combatant
                            entity_properties={character.entity_properties.clone()}
                            combatant_properties={character.combatant_properties.clone()}
                            />}).collect::<Html>()
                    }
                </div>
                if party.current_room.monsters.is_none() {
                    <PlayersReadyToExplore
                        players_ready={party.players_ready_to_explore.clone()}
                        players={party.player_usernames.clone()}
                    />
                }
            </div>
            if !game_state.viewing_inventory {
            <div class="w-1/2 border-l border-slate-400 p-2" >
                if party.current_room.room_type == DungeonRoomTypes::MonsterLair {
                    <MonsterLair
                        room={party.current_room.clone()}
                    />
                }
            </div>
            }
        </section>
    )
}
