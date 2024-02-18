mod ability_details;
use self::ability_details::ability_details;
use crate::components::common_components::atoms::targeting_indicator;
use crate::store::game_store::get_focused_character;
use crate::store::game_store::GameStore;
use common::combat::combat_actions::CombatAction;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combat_action: CombatAction,
}

#[function_component(ActionDetailsContextInfo)]
pub fn action_details_context_info(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let game = game_state.game.as_ref().expect("to be in a game");
    let focused_character =
        get_focused_character(&game_state).expect("to have a focused_character");
    let focused_character_id = focused_character.entity_properties.id;
    let combat_action_properties = props
        .combat_action
        .get_properties_if_owned(game, focused_character_id)
        .expect("to own the action");
    let action_name = match &props.combat_action {
        CombatAction::AbilityUsed(ability_name) => format!("{}", ability_name),
        CombatAction::ConsumableUsed(item_id) => {
            format!(
                "{}",
                focused_character
                    .combatant_properties
                    .inventory
                    .get_consumable(item_id)
                    .expect("to only look at owned items")
                    .consumable_type
            )
        }
    };

    let mut targeting_schemes = combat_action_properties.targeting_schemes.clone();
    targeting_schemes.sort();

    let targeting_schemes_text = targeting_schemes
        .iter()
        .enumerate()
        .map(|(i, scheme)| {
            let mut to_return = scheme.to_string();
            if i < combat_action_properties.targeting_schemes.len() - 1 {
                to_return.push_str(", ")
            }
            to_return
        })
        .collect::<String>();

    let ability_option = match &props.combat_action {
        CombatAction::AbilityUsed(ability_name) => focused_character
            .combatant_properties
            .get_ability_if_owned(&ability_name)
            .ok(),
        CombatAction::ConsumableUsed(_) => None,
    };

    let ability_details = match ability_option {
        Some(ability) => ability_details(
            &game,
            &ability,
            &combat_action_properties,
            &focused_character.combatant_properties,
            focused_character_id,
        ),
        None => html!(),
    };

    html!(
        <div class="max-w-full w-full h-full flex">
            <div class="h-full w-1/2 relative">
                    <span>
                        {action_name}
                    </span>
                    <div class="mr-2 mb-1 mt-1 h-[1px] bg-slate-400" />
                    {ability_details}
                    <div>{combat_action_properties.description}</div>
            </div>
            <div class="h-full w-1/2 relative pl-2">
                <span class="flex justify-between pr-2">
                    {"Useability"}
                </span>
                <div class="mr-2 mb-1 mt-1 h-[1px] bg-slate-400" />
                <div>
                    {"Valid targets: "}{combat_action_properties.valid_target_categories}
                </div>
                <div>
                    {"Targeting schemes: "}{targeting_schemes_text}
                </div>
                <div>
                    {"Usable "}{format!("{}", combat_action_properties.usability_context)}
                </div>
                <div class="opacity-50 fill-slate-400 h-40 absolute bottom-5 right-3">
                    <img src="public/img/equipment-icons/1h-sword-a.svg" class="h-40 filter" />
                </div>
            </div>
        </div>
    )
}
