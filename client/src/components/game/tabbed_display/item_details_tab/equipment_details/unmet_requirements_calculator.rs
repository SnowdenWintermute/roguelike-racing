use common::combatants::CombatAttributes;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub equipment_requirements: HashMap<CombatAttributes, u16>,
    pub entity_id: u32,
}

#[function_component(EquipmentDetails)]
pub fn unmet_requirements_calculator(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();

    let game = &game_state.deref().game.as_ref().expect("");
    let current_party_id = game_state.clone().current_party_id.expect("");
    let focused_character_id = game_state.clone().focused_character_id;
    let focused_character = get_character(*game, current_party_id, focused_character_id);
    let focused_character_combat_attributes = &focused_character
        .expect("")
        .combatant_properties
        .get_total_attributes();

    // SET UNMET REQUIREMENT FLAGS
    let cloned_focused_character_combat_attributes = focused_character_combat_attributes.clone();
    let cloned_equipment_requirements = props.equipment_requirements.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let entity_id = props.entity_id;
    use_effect_with(
        (
            cloned_focused_character_combat_attributes,
            entity_id,
            is_compared_item,
        ),
        move |(character_attributes, _, _)| {
            let mut unmet_requirement_attributes = HashSet::new();
            for (attribute, value) in &cloned_equipment_requirements {
                let character_attribute_option = character_attributes.get(attribute);
                match character_attribute_option {
                    Some(attr_value) => {
                        if *attr_value >= *value as u16 {
                            continue;
                        } else {
                            unmet_requirement_attributes.insert(attribute.clone())
                        }
                    }
                    None => unmet_requirement_attributes.insert(attribute.clone()),
                };
            }
            if unmet_requirement_attributes.len() > 0 {
                cloned_game_dispatch.reduce_mut(|store| {
                    store.considered_item_unmet_requirements = Some(unmet_requirement_attributes)
                })
            } else {
                cloned_game_dispatch
                    .reduce_mut(|store| store.considered_item_unmet_requirements = None)
            };

            move || {
                cloned_game_dispatch
                    .reduce_mut(|store| store.considered_item_unmet_requirements = None);
            }
        },
    );

    html!()
}
