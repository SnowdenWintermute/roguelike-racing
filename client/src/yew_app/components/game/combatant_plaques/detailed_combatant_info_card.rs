use crate::yew_app::components::game::context_dependant_information_display::combatant_details_context_info::CombatantDetailsContextInfo;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM_SMALL;
use crate::yew_app::store::game_store::{DetailableEntities, GameStore};
use common::packets::CharacterId;
use gloo_utils::window;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant_id: CharacterId,
    pub combatant_plaque_ref: NodeRef,
    pub info_button_is_hovered: UseStateHandle<bool>,
}

#[function_component(DetailedCombatantInfoCard)]
pub fn detailed_combatant_info_card(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let combatant_detailed_info_ref = use_node_ref();
    let detailed_info_card_position_style = use_state(|| "".to_string());

    let detailed_info_card = if let Some(detailable) = &game_state.hovered_entity {
        match detailable {
            DetailableEntities::Combatant(combatant_details) => {
                if combatant_details.entity_properties.id == props.combatant_id {
                    Some(html!(
                    <div class="border border-slate-400 bg-slate-700 p-2.5">
                        <CombatantDetailsContextInfo combatant_id={combatant_details.entity_properties.id}/>
                    </div>
                    ))
                } else {
                    None
                }
            }
            DetailableEntities::Item(_) => None,
        }
    } else {
        match &game_state.detailed_entity {
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
        }
    };

    let cloned_combatant_plaque_ref = props.combatant_plaque_ref.clone();
    let cloned_combatant_detailed_info_ref = combatant_detailed_info_ref.clone();
    let cloned_detailed_info_card_position_style = detailed_info_card_position_style.clone();
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
            let mut style = String::from("");
            if plaque_y - detailed_info_height < 0 {
                // put below
                style.push_str(&format!(
                    "bottom: 0px; transform: translateY(100%); padding-top: {SPACING_REM_SMALL}rem;"
                ));
            } else {
                // put above
                style.push_str(&format!(
                    "top: 0px; transform: translateY(-{}px); padding-bottom: {SPACING_REM_SMALL}rem;",
                    detailed_info_height
                ));
            }
            let window_width_i32 = window_width.clone().unwrap().as_f64().unwrap() as i32;

            if plaque_x as i32 + detailed_info_width as i32 > window_width_i32 {
                let overflow = window_width_i32 - (plaque_x as i32 + detailed_info_width as i32);
                style.push_str(&format!("right: -1px; transform(translateX(-100%))",))
            } else {
                style.push_str("left: -1px;");
            }

            cloned_detailed_info_card_position_style.set(style);
        }
    });

    let info_button_hovered_styles = if *props.info_button_is_hovered {
        "z-50"
    } else {
        ""
    };

    html!(
    <div class={format!("absolute box-border {}", info_button_hovered_styles)}
         style={format!("{}", *detailed_info_card_position_style)}
         ref={combatant_detailed_info_ref.clone()}
         >
        { detailed_info_card.unwrap_or_else(|| html!()) }
    </div>
    )
}
