mod hp_and_mp;
mod weapon_damage;
use crate::components::client_consts::UNMET_REQUIREMENT_TEXT_COLOR;
use crate::components::game::character_sheet::character_attributes::weapon_damage::CharacterSheetWeaponDamage;
use crate::store::game_store::GameStore;
use common::combatants::combat_attributes::CombatAttributes;
use common::combatants::CombatantProperties;
use common::primatives::EntityProperties;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub combatant_properties: CombatantProperties,
    pub entity_properties: EntityProperties,
}

#[function_component(CharacterAttributes)]
pub fn character_attributes(props: &Props) -> Html {
    let Props {
        combatant_properties,
        entity_properties,
    } = props;
    let (game_state, _) = use_store::<GameStore>();
    let total_attributes = combatant_properties.get_total_attributes();
    let mut combatant_attributes_as_vec = total_attributes
        .iter()
        .filter(|(attribute, _)| !is_custom_displayed_attribute(&attribute))
        .collect::<Vec<(&CombatAttributes, &u16)>>();
    combatant_attributes_as_vec.sort_by(move |a, b| a.0.partial_cmp(&b.0).unwrap());
    let num_attributes = combatant_attributes_as_vec.len();
    let half_num_attributes = match num_attributes % 2 {
        0 => num_attributes / 2,
        _ => (num_attributes - 1) / 2,
    };

    html!(
        <div class="h-full pl-2 w-1/2">
            <div class="font-bold" >
                {entity_properties.name.clone()}{format!(" ({})", combatant_properties.combatant_class)}
                {format!(" {}", entity_properties.id)}
            </div>
            <div class="flex mb-1" >
                <ul class="list-none w-1/2 mr-1" >
                    {combatant_attributes_as_vec.iter()
                        .enumerate()
                        .filter(|( i, _ )| i < &half_num_attributes)
                        .map(|(_, (attribute, value))| attribute_list_item(attribute, value, &game_state)).collect::<Html>()}
                </ul>
                <ul class="list-none w-1/2 ml-1" >
                    {combatant_attributes_as_vec.iter()
                        .enumerate()
                        .filter(|( i, _)| i >= &half_num_attributes)
                        .map(|(_, (attribute, value))| attribute_list_item(attribute, value, &game_state)).collect::<Html>()}
                </ul>
            </div>
            <div id="divider" class="bg-slate-400 h-[1px] flex mt-2 mr-2 ml-2 mb-2" />
            {hp_and_mp::hp_and_mp(&combatant_properties, &total_attributes)}
            <CharacterSheetWeaponDamage combatant_id={entity_properties.id} />
        </div>
    )
}

fn is_custom_displayed_attribute(attribute: &CombatAttributes) -> bool {
    attribute == &CombatAttributes::Hp || attribute == &CombatAttributes::Mp
}

fn attribute_list_item(
    attribute: &CombatAttributes,
    value: &u16,
    game_state: &Rc<GameStore>,
) -> VNode {
    let is_unmet_requirement = match &game_state.considered_item_unmet_requirements {
        Some(unmet_attribute_requirements) => unmet_attribute_requirements.get(attribute).is_some(),
        None => false,
    };

    let highlight_class = if is_unmet_requirement {
        UNMET_REQUIREMENT_TEXT_COLOR
    } else {
        ""
    };

    html!(
        <li class={ format!( "flex justify-between {}", highlight_class  ) }>
            <span>{format!("{}", attribute)}</span>
            <span>{format!("{}", value)}</span>
        </li>
    )
}
