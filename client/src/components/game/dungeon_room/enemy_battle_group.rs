use crate::components::game::dungeon_room::combatant::Combatant;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
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
    // let game_option = ;

    let battle_option = get_current_battle_option(&game_state);
    let enemy_battle_group_option = if let Some(battle) = battle_option {
        let battle_groups_result =
            battle.get_ally_and_enemy_battle_groups(&props.ally_combatant_id);
        if let Ok((_, enemy_battle_group)) = battle_groups_result {
            Some(enemy_battle_group)
        } else {
            None
        }
    } else {
        None
    };

    if let Some(enemy_battle_group) = enemy_battle_group_option {
        let enemy_combatants = 
        html!(
            <div class="flex flex-col items-end whitespace-nowrap" >
                {monsters.iter().map(|monster| {
                    html!(
                        <Combatant
                            entity_properties={monster.entity_properties.clone()}
                            combatant_properties={monster.combatant_properties.clone()}
                        />
                        )
                   }).collect::<Html>()}
            </div>
        )
    } else {
        html!(<div>{ "error - no enemy combatant battle group" }</div>)
    }
}
