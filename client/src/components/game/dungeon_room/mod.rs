use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::game::dungeon_room::empty_room::EmptyRoom;
use crate::components::game::dungeon_room::stairs::Stairs;
use common::character::Character;
use common::dungeon_rooms::DungeonRoomTypes;
mod empty_room;
mod enemy_battle_group;
mod items_on_ground;
mod players_ready_to_explore;
mod stairs;
use crate::components::game::combatant::Combatant;
use crate::components::game::dungeon_room::enemy_battle_group::EnemyBattleGroup;
use crate::components::game::dungeon_room::items_on_ground::ItemsOnGround;
use crate::components::game::dungeon_room::players_ready_to_explore::PlayersReadyToExplore;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub party_id: u32,
}

#[function_component(DungeonRoom)]
pub fn dungeon_room(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let game_result = game_state.get_current_game();
    if let Ok(game) = game_result {
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
            true => "w-[22rem] mr-4",
            false => "w-full",
        };

        let time_of_death_option = if let Some(time_of_wipe) = party.time_of_wipe {
            Some(format!("unix timestamp ({})", time_of_wipe))
        } else {
            None
        };
        let leave_game = Callback::from(move |_| {
            game_dispatch.set(GameStore::default());
        });
        let players_ready_to_descend_option =
            if party.current_room.room_type == DungeonRoomTypes::Stairs {
                Some(party.players_ready_to_descend.clone())
            } else {
                None
            };

        // used to determine which battle group is the enemy
        let ally_combatant_id = characters.first().expect("party to have characters").0;

        html!(
            <section class={format!("h-full border border-slate-400 bg-slate-700 flex {}", conditional_styles)} >
                <div class="w-fit flex p-2 h-full" >
                    <div class="flex flex-col mr-4 h-full flex-grow" >
                        {characters.iter().map(|(_id, character)|
                            html!{<Combatant
                                entity_properties={character.entity_properties.clone()}
                                combatant_properties={character.combatant_properties.clone()}
                                />}).collect::<Html>()
                        }
                    </div>
                    if party.current_room.monsters.is_none() && !game_state.viewing_inventory {
                        <PlayersReadyToExplore
                            players_ready_to_explore={party.players_ready_to_explore.clone()}
                            players_ready_to_descend_option={players_ready_to_descend_option}
                            players={party.player_usernames.clone()}
                        />
                    }
                </div>
                <div class="border-l border-slate-400 p-2 flex flex-col flex-grow w-full" >
                    if let Some(time_of_death) = time_of_death_option {
                        <div class=" border border-slate-400 bg-slate-700 p-4
                            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" >
                            <span class="
                            text-lg mb-2
                            ">{"Time of death: "}{time_of_death}</span>
                            <ButtonBasic onclick={leave_game}>
                                {"Leave Game"}
                            </ButtonBasic>
                        </div>
                    }
                    if party.current_room.room_type == DungeonRoomTypes::Stairs && !game_state.viewing_inventory {
                        <Stairs />
                    }
                    if party.current_room.room_type == DungeonRoomTypes::Empty && !game_state.viewing_inventory {
                        <EmptyRoom />
                    }
                    if let Some(battle_id) = game_state.current_battle_id {
                        <EnemyBattleGroup battle_id={battle_id} ally_combatant_id={ally_combatant_id} />
                    } else {
                        <ItemsOnGround />
                    }
                </div>
                // }
            </section>
        )
    } else {
        html!({ "error - no game found" })
    }
}
