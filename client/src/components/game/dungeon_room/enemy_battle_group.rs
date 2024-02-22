use crate::components::game::combatant::Combatant;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
use common::combatants::CombatantProperties;
use common::errors::AppError;
use common::primatives::EntityProperties;
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

    let inventory_open_styles = if game_state.viewing_inventory {
        "w-full"
    } else {
        ""
    };

    if let Some(enemy_combatants) = enemy_combatants_option {
        html!(
            <div class={format!("flex flex-col items-end whitespace-nowrap {}", inventory_open_styles)} >
                {enemy_combatants.iter().map(|combatant_result| {
                    if let Ok((entity_properties ,combatant_properties )) = *combatant_result {
                        let entity_properties = entity_properties.clone();
                        let combatant_properties = combatant_properties.clone();
                        html!(
                            <Combatant
                                entity_properties={entity_properties}
                                combatant_properties={combatant_properties}
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
