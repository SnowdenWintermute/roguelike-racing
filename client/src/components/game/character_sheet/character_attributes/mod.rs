mod hp_and_mp;
pub mod weapon_damage;
use crate::components::client_consts::UNMET_REQUIREMENT_TEXT_COLOR;
use crate::components::common_components::atoms::divider::Divider;
use crate::components::common_components::atoms::hoverable_tooltip_wrapper::HoverableTooltipWrapper;
use crate::components::game::character_sheet::character_attributes::weapon_damage::CharacterSheetWeaponDamage;
use crate::components::game::combatant::combatant_class_icon::CombatantClassIcon;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::websocket_store::WebsocketStore;
use common::combat::hp_change_source_types::MeleeOrRanged;
use common::combatants::combat_attributes::CombatAttributes;
use common::combatants::combat_attributes::ATTRIBUTE_POINT_ASSIGNABLE_ATTRIBUTES;
use common::combatants::CombatantControlledBy;
use common::combatants::CombatantProperties;
use common::items::equipment::EquipmentProperties;
use common::items::equipment::EquipmentSlots;
use common::items::equipment::EquipmentTypes;
use common::packets::client_to_server::PlayerInputs;
use common::primatives::EntityProperties;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub combatant_properties: CombatantProperties,
    pub entity_properties: EntityProperties,
    #[prop_or(false)]
    pub show_attribute_assignment_buttons: bool,
}

#[function_component(CharacterAttributes)]
pub fn character_attributes(props: &Props) -> Html {
    let Props {
        combatant_properties,
        entity_properties,
        show_attribute_assignment_buttons,
    } = props;
    let (game_state, _) = use_store::<GameStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();

    let player_owns_character = if let Ok(party) = game_state.get_current_party() {
        party.player_owns_character(&lobby_state.username, entity_properties.id)
    } else {
        false
    };

    let mh_equipment_option = combatant_properties.get_equipped_item(&EquipmentSlots::MainHand);
    // check for sheilds since they can't be used to attack
    let mh_weapon_option = EquipmentProperties::get_weapon_equipment_properties_option_from_equipment_properties_option(mh_equipment_option);
    let weapon_type_equipped = match mh_weapon_option {
        Some(equipment_properties) => match equipment_properties.equipment_type {
            EquipmentTypes::TwoHandedRangedWeapon(_, _) => MeleeOrRanged::Ranged,
            _ => MeleeOrRanged::Melee,
        },
        None => MeleeOrRanged::Melee,
    };

    let mut total_attributes = combatant_properties.get_total_attributes();
    let armor_pen_attribute_bonus_based_on_weapon_type = match weapon_type_equipped {
        MeleeOrRanged::Melee => {
            CombatantProperties::get_armor_pen_derrived_attribute_based_on_weapon_type(
                &total_attributes,
                &CombatAttributes::Strength,
            )
        }
        MeleeOrRanged::Ranged => {
            CombatantProperties::get_armor_pen_derrived_attribute_based_on_weapon_type(
                &total_attributes,
                &CombatAttributes::Dexterity,
            )
        }
    };
    let total_armor_pen = total_attributes
        .entry(CombatAttributes::ArmorPenetration)
        .or_insert(0);
    *total_armor_pen += armor_pen_attribute_bonus_based_on_weapon_type;

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

    let exp_required_for_next_level_string = match combatant_properties
        .experience_points
        .required_for_next_level
    {
        Some(number) => format!("{number}"),
        None => "∞".to_string(),
    };

    let has_unspent_attribute_points = combatant_properties.unspent_attribute_points > 0;
    let has_unspent_ability_points = combatant_properties.unspent_ability_points > 0;
    let unspent_attribute_points_display = if has_unspent_attribute_points
        && combatant_properties.controlled_by != CombatantControlledBy::AI
        && *show_attribute_assignment_buttons
    {
        html!(
            <li class={"text-ffxipink flex justify-between"}>
                <span>{{"Unspent: "}}</span>
                <span>
                    <span>{combatant_properties.unspent_attribute_points}</span>
                </span>
            </li>
        )
    } else {
        html!()
    };
    let unspent_ability_points_display = if has_unspent_ability_points
        && combatant_properties.controlled_by != CombatantControlledBy::AI
    {
        html!(
            <div class="text-ffxipink" >{"unspent ability points: "}{combatant_properties.unspent_ability_points}</div>
        )
    } else {
        html!()
    };

    let experience_points_text = if combatant_properties.controlled_by != CombatantControlledBy::AI
    {
        {
            format!(
                "{} / {} experience",
                combatant_properties.experience_points.current, exp_required_for_next_level_string
            )
        }
    } else {
        String::from("")
    };

    html!(
        <div class="h-full pl-2 w-1/2">
            <div class="font-bold flex justify-between items-center" >
                <span>
                    {entity_properties.name.clone()}
                    {format!(" ({})", combatant_properties.combatant_class)}
                </span>
                <span class="h-10 w-10 p-1 flex justify-center rotate-45" >
                    <CombatantClassIcon combatant_class={combatant_properties.combatant_class.clone()} />
                </span>
            </div>
            <div class="flex justify-between" >
                <span>
                    {"Level "}{combatant_properties.level}{" "}
                </span>
                <span>
                    {experience_points_text}
                </span>
            </div>
            <Divider styles={AttrValue::from("mr-2 ml-2 ")} />
            <div class="flex mb-1" >
                <ul class="list-none w-1/2 mr-1" >
                    {combatant_attributes_as_vec.iter()
                        .enumerate()
                        .filter(|( i, _ )| i < &half_num_attributes)
                        .map(|(_, (attribute, value))|
                             attribute_list_item(
                                     attribute,
                                     value,
                                     &game_state,
                                     has_unspent_attribute_points,
                                     &websocket_state,
                                     player_owns_character,
                                     *show_attribute_assignment_buttons,
                                 )).collect::<Html>()}
                    {unspent_attribute_points_display}
                </ul>
                <ul class="list-none w-1/2 ml-1" >
                    {combatant_attributes_as_vec.iter()
                        .enumerate()
                        .filter(|( i, _)| i >= &half_num_attributes)
                        .map(|(_, (attribute, value))|
                             attribute_list_item(
                                     attribute,
                                     value,
                                     &game_state,
                                     has_unspent_attribute_points,
                                     &websocket_state,
                                     player_owns_character,
                                     *show_attribute_assignment_buttons,
                                 )).collect::<Html>()}
                </ul>
            </div>
            <Divider styles={AttrValue::from("mr-2 ml-2 ")} />
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
    has_unspent_attribute_points: bool,
    websocket_state: &Rc<WebsocketStore>,
    player_owns_character: bool,
    show_attribute_assignment_buttons: bool,
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

    let focused_character_id = game_state.focused_character_id;

    let increase_attribute_button = if has_unspent_attribute_points
        && ATTRIBUTE_POINT_ASSIGNABLE_ATTRIBUTES.contains(attribute)
        && player_owns_character
        && show_attribute_assignment_buttons
    {
        let cloned_websocket_state = websocket_state.clone();
        let cloned_attribute = attribute.clone();
        let handle_click = Callback::from(move |_| {
            send_client_input(
                &cloned_websocket_state.websocket,
                PlayerInputs::IncrementAttribute(focused_character_id, cloned_attribute),
            )
        });

        html!(
        <button
            onclick={handle_click}
            class="inline-block h-4 w-4 border border-slate-400 text-lg leading-3 ml-2"
        >
            {"+"}
        </button>
        )
    } else {
        html!()
    };

    html!(
        <li class={ format!( "flex justify-between {}", highlight_class  ) }>
            <span>
                <span class="inline-block h-6 w-6" >
                    <HoverableTooltipWrapper tooltip_text={AttrValue::from(attribute.get_description().to_string())} >
                        <span class="cursor-help h-full w-full inline-block" >
                            {"ⓘ "}
                        </span>
                    </HoverableTooltipWrapper>
                </span>
                {format!("{}", attribute)}
            </span>
            <span>
                <span>{format!("{}", value)}</span>
                {increase_attribute_button}
            </span>
        </li>
    )
}
