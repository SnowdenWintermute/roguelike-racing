mod ability_details;
use self::ability_details::ability_details;
use crate::yew_app::store::game_store::get_focused_character;
use crate::yew_app::store::game_store::GameStore;
use common::combat::combat_actions::CombatAction;
use common::items::ItemProperties;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combat_action: CombatAction,
    pub hide_title: bool,
}

#[function_component(ActionDetailsContextInfo)]
pub fn action_details_context_info(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let game = game_state.game.as_ref().expect("to be in a game");
    let focused_character =
        get_focused_character(&game_state).expect("to have a focused_character");
    let focused_character_id = focused_character.entity_properties.id;
    let party_id = game_state.current_party_id.expect("to be in a party");
    let combat_action_properties = props
        .combat_action
        .get_properties(game, focused_character_id, party_id)
        .expect("to either own the ability or have the consumable exist in the party");
    let action_name = match &props.combat_action {
        CombatAction::AbilityUsed(ability_name) => format!("{}", ability_name),
        CombatAction::ConsumableUsed(item_id) => {
            let item = game
                .get_item_in_adventuring_party(party_id, *item_id)
                .expect("for an item to be in this party");
            match &item.item_properties {
                ItemProperties::Consumable(consumable_properties) => {
                    format!("{}", consumable_properties.consumable_type)
                }
                ItemProperties::Equipment(_) => {
                    "Equipment can not be used as an action".to_string()
                }
            }
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
        <div>
            if !props.hide_title {
                <span>
                {action_name}
                </span>
                    <div class="mr-2 mb-1 mt-1 h-[1px] bg-slate-400" />
            }
            {ability_details}
            <div>{combat_action_properties.description}</div>
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
    )
}
