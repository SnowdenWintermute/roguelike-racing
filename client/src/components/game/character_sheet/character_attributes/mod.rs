use common::{
    combatants::{CombatAttributes, CombatantProperties},
    primatives::EntityProperties,
};
use yew::{prelude::*, virtual_dom::VNode};

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
    let mut combatant_attributes_as_vec = combatant_properties
        .get_total_attributes()
        .into_iter()
        .filter(|(attribute, _)| !is_custom_displayed_attribute(&attribute))
        .collect::<Vec<(CombatAttributes, u16)>>();
    combatant_attributes_as_vec.sort_by(move |a, b| a.0.partial_cmp(&b.0).unwrap());
    let num_attributes = combatant_attributes_as_vec.len();
    let half_num_attributes = match num_attributes % 2 {
        0 => num_attributes / 2,
        _ => (num_attributes - 1) / 2,
    };

    html!(
        <div class="h-full pl-2 w-1/2">
            <div class="font-bold" >{entity_properties.name.clone()}</div>
            <div class="flex" >
                <ul class="list-none w-1/2 mr-1" >
                    {combatant_attributes_as_vec.iter()
                        .enumerate()
                        .filter(|( i, _ )| i < &half_num_attributes)
                        .map(|(_, (attribute, value))| attribute_list_item(attribute, value)).collect::<Html>()}
                </ul>
                <ul class="list-none w-1/2 ml-1" >
                    {combatant_attributes_as_vec.iter()
                        .enumerate()
                        .filter(|( i, _)| i >= &half_num_attributes)
                        .map(|(_, (attribute, value))| attribute_list_item(attribute, value)).collect::<Html>()}
                </ul>
            </div>
            <div>

            </div>
        </div>
    )
}

fn is_custom_displayed_attribute(attribute: &CombatAttributes) -> bool {
    attribute == &CombatAttributes::Hp || attribute == &CombatAttributes::Mp
}

fn attribute_list_item(attribute: &CombatAttributes, value: &u16) -> VNode {
    html!(
        <li class="flex justify-between" >
            <span>{format!("{}", attribute)}</span>
            <span>{format!("{}", value)}</span>
        </li>
    )
}
