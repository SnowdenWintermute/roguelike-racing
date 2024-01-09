use crate::components::game::dungeon_room::combatant::Combatant;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
use common::combatants::CombatantProperties;
use common::errors::AppError;
use common::primatives::EntityProperties;
use gloo::console::log;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub battle_id: u32,
    pub ally_combatant_id: u32,
}

#[function_component(EnemyBattleGroup)]
pub fn enemy_battle_group(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let game = game_state.game.as_ref().expect("to be in game");

    let battle_option = get_current_battle_option(&game_state);
    let enemy_combatants_option = if let Some(battle) = battle_option {
        let battle_groups_result =
            battle.get_ally_and_enemy_battle_groups(&props.ally_combatant_id);
        if let Ok((_, enemy_battle_group)) = battle_groups_result {
            let enemy_combatant_results = enemy_battle_group
                .combatant_ids
                .iter()
                .map(|id| game.get_combatant_in_battle_by_id(battle, id))
                .collect::<Vec<Result<(&EntityProperties, &CombatantProperties), AppError>>>();
            Some(enemy_combatant_results)
        } else {
            None
        }
    } else {
        None
    };

    if let Some(enemy_combatants) = enemy_combatants_option {
        html!(
            <div class="flex flex-col items-end whitespace-nowrap" >
                {enemy_combatants.iter().map(|combatant_result| {
                    if let Ok(combatant) = combatant_result {
                        html!(
                            <Combatant
                                entity_properties={combatant.0.clone()}
                                combatant_properties={combatant.1.clone()}
                            />
                            )
                    } else {
                        html!({"no combatant found with the provided id"})
                    }
                   }).collect::<Html>()}
            </div>
        )
    } else {
        html!(<div>{ "error - no enemy combatant battle group" }</div>)
    }
}
