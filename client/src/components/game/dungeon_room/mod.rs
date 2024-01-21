use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::websocket_store::WebsocketStore;
use common::character::Character;
use common::packets::client_to_server::PlayerInputs;
use gloo::console::log;
use wasm_bindgen::JsValue;
mod enemy_battle_group;
mod items_on_ground;
mod players_ready_to_explore;
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
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (game_state, _) = use_store::<GameStore>();
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
            true => "min-w-[350px] w-[350px] mr-4",
            false => "w-full",
        };

        let time_of_death_option = if let Some(time_of_wipe) = party.time_of_wipe {
            let js_date = js_sys::Date::new_0();
            log!(format!("time of wipe: {:#?}", time_of_wipe));
            let as_seconds = time_of_wipe / 1000;
            log!(format!("as_seconds: {:#?}", as_seconds));
            js_date.set_time(as_seconds as f64);
            let js_value = wasm_bindgen::JsValue::from(as_seconds);
            let js_date = js_sys::Date::from(js_value);
            log!(format!("js date: {:#?}", js_date));
            let formatted_time = js_date.to_locale_string("en-US", &JsValue::from("{}"));
            log!(format!("js date: {:#?}", formatted_time));
            Some(formatted_time)
        } else {
            None
        };
        let leave_game = Callback::from(move |_| {
            send_client_input(&websocket_state.websocket, PlayerInputs::LeaveGame)
        });

        // used to determine which battle group is the enemy
        let ally_combatant_id = characters.first().expect("party to have characters").0;

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
                    if let Some(battle_id) = game_state.current_battle_id {
                        <EnemyBattleGroup battle_id={battle_id} ally_combatant_id={ally_combatant_id} />
                    } else {
                        <ItemsOnGround />
                    }
                </div>
                }
            </section>
        )
    } else {
        html!({ "error - no game found" })
    }
}
