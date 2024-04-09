mod combatant_info_button;
pub mod combatant_plaque_group;
mod combatant_value_bars;
mod detailed_combatant_info_card;
mod focus_character_button;
use crate::yew_app::components::common_components::atoms::targeting_indicator::TargetingIndicator;
use crate::yew_app::components::game::combatant::combatant_is_selected::combatant_is_selected;
use crate::yew_app::components::game::combatant::combatant_is_targeted::combatant_targeted_by;
use crate::yew_app::components::game::combatant_plaques::combatant_info_button::CombatantInfoButton;
use crate::yew_app::components::game::combatant_plaques::combatant_value_bars::get_combatant_value_bars;
use crate::yew_app::components::game::combatant_plaques::detailed_combatant_info_card::DetailedCombatantInfoCard;
use crate::yew_app::components::game::combatant_plaques::focus_character_button::FocusCharacterButton;
use crate::yew_app::store::game_store::GameStore;
use common::packets::CharacterId;
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
    let combatant_id = props.combatant_id;
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let portrait_height = use_state(|| 0);
    let combatant_plaque_ref = use_node_ref();
    let name_and_bars_ref = use_node_ref();
    let info_button_is_hovered = use_state(|| false);

    let cloned_node_ref = name_and_bars_ref.clone();
    let cloned_portrait_height = portrait_height.clone();
    use_effect_with((), move |_| {
        let element_option = cloned_node_ref.cast::<HtmlElement>();
        if let Some(element) = element_option {
            let height = element.client_height();
            cloned_portrait_height.set(height)
        }
    });

    let game = game_state.game.as_ref().expect("to be in a game");

    let (entity_properties, combatant_properties) = game
        .get_combatant_by_id(&props.combatant_id)
        .expect("to have a reference to a valid combatant");

    let (hp_bar, mp_bar, experience_bar) = get_combatant_value_bars(&combatant_properties);

    let is_selected = combatant_is_selected(game_state.clone(), props.combatant_id);
    let targeted_by = combatant_targeted_by(game_state.clone(), &props.combatant_id);

    let cloned_game_dispatch = game_dispatch.clone();
    let handle_unspent_attributes_button_click = Callback::from(move |_: MouseEvent| {
        cloned_game_dispatch.reduce_mut(|store| {
            store.focused_character_id = combatant_id;
            store.viewing_attribute_point_assignment_menu = true;
        })
    });

    let unspent_attributes_button = if combatant_properties.unspent_attribute_points > 0 {
        html!(
            <button onclick={handle_unspent_attributes_button_click}
                class="bg-ffxipink h-5 w-5 border border-slate-950 text-slate-950 text-lg leading-3 ml-1" >
                { "+" }
            </button>
        )
    } else {
        html!()
    };

    let focused_class = if *info_button_is_hovered {
        "border-white"
    } else if game_state.focused_character_id == props.combatant_id {
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
        ref={combatant_plaque_ref.clone()}
        >
        {targeting_indicators}
         <DetailedCombatantInfoCard
             combatant_id={combatant_id}
             combatant_plaque_ref={combatant_plaque_ref.clone()}
             info_button_is_hovered={info_button_is_hovered.clone()}
         />
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
                    {unspent_attributes_button}
                </span>
                <span>
                    <CombatantInfoButton combatant_id={combatant_id} info_button_is_hovered={info_button_is_hovered.clone()} />
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
