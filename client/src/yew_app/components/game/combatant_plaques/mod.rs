pub mod combatant_plaque_group;
mod focus_character_button;
use crate::yew_app::{
    components::{
        common_components::atoms::targeting_indicator::TargetingIndicator,
        game::{
            combatant::{
                combatant_is_selected::combatant_is_selected,
                combatant_is_targeted::combatant_targeted_by, value_bar::ValueBar,
            },
            combatant_plaques::focus_character_button::FocusCharacterButton,
            context_dependant_information_display::combatant_details_context_info::CombatantDetailsContextInfo,
        },
    },
    store::game_store::{CombatantDetails, DetailableEntities, GameStore},
};
use common::{combatants::combat_attributes::CombatAttributes, packets::CharacterId};
use gloo::console::log;
use gloo_utils::window;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub combatant_id: CharacterId,
    pub show_experience: bool,
}

#[function_component(CombatantPlaque)]
pub fn combatant_plaque(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let portrait_height = use_state(|| 0);
    let combatant_plaque_ref = use_node_ref();
    let combatant_detailed_info_ref = use_node_ref();
    let name_and_bars_ref = use_node_ref();
    let detailed_info_card_position_style = use_state(|| "".to_string());

    let cloned_node_ref = name_and_bars_ref.clone();
    let cloned_portrait_height = portrait_height.clone();
    use_effect_with((), move |_| {
        let element_option = cloned_node_ref.cast::<HtmlElement>();
        if let Some(element) = element_option {
            let height = element.client_height();
            cloned_portrait_height.set(height)
        }
    });

    let detailed_info_card = match &game_state.detailed_entity {
        Some(detailable) => match detailable {
            DetailableEntities::Combatant(combatant_details) => {
                if combatant_details.entity_properties.id == props.combatant_id {
                    Some(html!(
                    <div class="border border-slate-400 bg-slate-700 p-2.5 ">
                        <CombatantDetailsContextInfo combatant_id={combatant_details.entity_properties.id}/>
                    </div>
                    ))
                } else {
                    None
                }
            }
            DetailableEntities::Item(_) => None,
        },
        None => None,
    };

    let cloned_combatant_plaque_ref = combatant_plaque_ref.clone();
    let cloned_combatant_detailed_info_ref = combatant_detailed_info_ref.clone();
    let cloned_detailed_info_card_position_style = detailed_info_card_position_style.clone();
    // @TODO - just track the id changed
    let plaque_option = cloned_combatant_plaque_ref.cast::<HtmlElement>();
    let detailed_info_option = cloned_combatant_detailed_info_ref.cast::<HtmlElement>();
    let detailed_entity = game_state.detailed_entity.clone();
    let detailed_info_height = match detailed_info_option {
        Some(element) => Some(element.client_height().clone()),
        None => None,
    };
    use_effect_with(detailed_info_card.is_some(), move |_| {
        let plaque_option = cloned_combatant_plaque_ref.cast::<HtmlElement>();
        let detailed_info_option = cloned_combatant_detailed_info_ref.cast::<HtmlElement>();
        if plaque_option.is_some() && detailed_info_option.is_some() {
            let window_height = window().inner_height();
            let window_width = window().inner_height();
            let detailed_info_width = detailed_info_option.as_ref().unwrap().client_width();
            let detailed_info_height = detailed_info_option.as_ref().unwrap().client_height();
            let plaque_y = plaque_option
                .as_ref()
                .unwrap()
                .get_bounding_client_rect()
                .y() as i32;
            let plaque_width = plaque_option.as_ref().unwrap().client_width();
            let plaque_height = plaque_option.as_ref().unwrap().client_height();
            let plaque_x = plaque_option
                .as_ref()
                .unwrap()
                .get_bounding_client_rect()
                .x() as i32;
            log!(format!("detailed_info_height: {detailed_info_height}"));
            log!(format!("plaque_y: {plaque_y}"));
            // log!(format!(": {plaque_y}"));
            if plaque_y - detailed_info_height < 0 {
                log!("putting below");
                // put below
                cloned_detailed_info_card_position_style.set(format!(
                    "left: 0px; bottom: 0px; transform: translateY(100%);"
                ));
            } else {
                // put above
                cloned_detailed_info_card_position_style.set(format!(
                    "left: 0px; top: 0px; transform: translateY(-{}px)",
                    detailed_info_height
                ));
            }
        }
        // if plaque_x + detailed_info_width > window_width {
        //     // calculate
        // }
    });

    let game = game_state.game.as_ref().expect("to be in a game");

    let (entity_properties, combatant_properties) = game
        .get_combatant_by_id(&props.combatant_id)
        .expect("to have a reference to a valid combatant");

    let total_attributes = combatant_properties.get_total_attributes();
    let max_hp_option = total_attributes.get(&CombatAttributes::Hp);
    let max_mp_option = total_attributes.get(&CombatAttributes::Mp);

    let cloned_entity_properties = entity_properties.clone();
    let cloned_combatant_properties = combatant_properties.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let handle_info_click = Callback::from(move |_| {
        cloned_game_dispatch.reduce_mut(|store| {
            store.detailed_entity = Some(DetailableEntities::Combatant(CombatantDetails {
                entity_properties: cloned_entity_properties.clone(),
                combatant_properties: cloned_combatant_properties.clone(),
            }));
        });
    });

    let hp_bar = if let Some(max_hp) = max_hp_option {
        html!(<ValueBar max={max_hp} curr={combatant_properties.hit_points} color={"green-700"} />)
    } else {
        html!({ "Immortal Object" })
    };

    let mp_bar = if let Some(max_mp) = max_mp_option {
        if *max_mp == 0 {
            html!()
        } else {
            html!(<ValueBar max={max_mp} curr={combatant_properties.mana} color={"blue-700"} />)
        }
    } else {
        html!({ "Infinite Mana" })
    };

    let experience_bar = {
        if let Some(required_exp_to_level) = combatant_properties
            .experience_points
            .required_for_next_level
        {
            html!(
                <ValueBar max={required_exp_to_level}
                          curr={combatant_properties.experience_points.current}
                          color={"ffxipink"}
                          hide_numbers={ true } />
            )
        } else {
            html!()
        }
    };

    let is_selected = combatant_is_selected(game_state.clone(), props.combatant_id);
    let targeted_by = combatant_targeted_by(game_state.clone(), &props.combatant_id);

    let focused_class = if game_state.focused_character_id == props.combatant_id {
        "border-lime-500"
    } else if is_selected {
        "border-yellow-400"
    } else {
        "border-slate-400 "
    };

    let targeting_indicators = if targeted_by.len() > 0 {
        html!(
            <div class="absolute top-[-1.5rem] left-1/2 -translate-x-1/2 z-20 flex" >
                {targeted_by.iter().map(|combatant_id_and_with_what| html!(
                    <TargetingIndicator
                    combat_action={combatant_id_and_with_what.1.clone()}
                    />
                )).collect::<Html>()}
            </div>)
    } else {
        html!()
    };

    html!(
    <div class={format!("w-96 h-fit border bg-slate-700 pointer-events-auto flex p-2.5 {focused_class} relative box-border")}
        ref={combatant_plaque_ref}
        >
        {targeting_indicators}
        <div class="absolute box-border"
             style={format!("{}", *detailed_info_card_position_style)}
             ref={ combatant_detailed_info_ref }
             >
            {detailed_info_card.unwrap_or_else( || html!())}
        </div>
        <div class="h-full aspect-square mr-2 border border-slate-400 bg-slate-600 rounded-full relative"
             style={format!("height: {}px;", *portrait_height)}
        >
            <div class="absolute -bottom-1 left-1/2 -translate-x-1/2 h-5 border border-slate-400 bg-slate-700 pr-2 pl-2 text-sm flex items-center justify-center">
                {combatant_properties.level}
            </div>
        </div>
        <div class="flex-grow"
        ref={name_and_bars_ref}
        >
            <div class="mb-1.5 flex justify-between text-lg">
                <span>
                    {entity_properties.name.clone()}
                </span>
                <span>
                    <button onclick={handle_info_click} class="hover:bg-slate-950 hover:border-slate-400 rounded-full leading-4">
                        {"ⓘ "}
                    </button>
                </span>
            </div>
            <div class="h-5 mb-1">
                {hp_bar}
            </div>
            <div class="h-5">
                {mp_bar}
            </div>
            if props.show_experience {
                <div class="h-5 mt-1 flex text-sm">
                    <FocusCharacterButton id={props.combatant_id} />
                    {experience_bar}
                </div>
            }
        </div>
    </div>
    )
}
