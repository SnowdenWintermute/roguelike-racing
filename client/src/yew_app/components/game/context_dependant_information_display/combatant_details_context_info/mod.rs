use crate::yew_app::components::common_components::atoms::button_basic::ButtonBasic;
use crate::yew_app::components::common_components::atoms::divider::Divider;
use crate::yew_app::components::common_components::atoms::hoverable_tooltip_wrapper::HoverableTooltipWrapper;
use crate::yew_app::components::game::character_sheet::character_attributes::CharacterAttributes;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant_id: u32,
}

#[function_component(CombatantDetailsContextInfo)]
pub fn combatant_details_context_info(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let Props { combatant_id } = props;
    let game_result = game_state.get_current_game();
    let combatant_option = if let Ok(game) = game_result {
        game.get_combatant_by_id(&combatant_id).ok()
    } else {
        None
    };

    let close_display = Callback::from(move |_| {
        game_dispatch.reduce_mut(|store| {
            store.detailed_entity = None;
            store.hovered_entity = None
        });
    });

    if let Some((entity_properties, combatant_properties)) = combatant_option {
        html!(
            <div class="flex justify-between ">
                <CharacterAttributes
                    combatant_properties={combatant_properties.clone()}
                    entity_properties={entity_properties.clone()}
                />
                <div class="h-full pl-4 w-1/2" >
                    <div class="w-full flex justify-end" >
                        <ButtonBasic onclick={close_display} >{"Close"}</ButtonBasic>
                    </div>
                    <div class="flex justify-between" >
                        <span>
                            {"Traits "}
                        </span>
                        <span>
                            {" "}
                        </span>
                    </div>
                    <Divider  />
                    <ul>
                        {combatant_properties.traits.iter().map(|item| html!(
                            <li>
                                <span class="inline-block h-6 w-6">
                                    <HoverableTooltipWrapper tooltip_text={AttrValue::from(item.get_description().to_string())} >
                                        <span class="cursor-help h-full w-full inline-block">
                                            {"ⓘ "}
                                        </span>
                                    </HoverableTooltipWrapper>
                                </span>
                                {format!("{}", item)}
                            </li>
                        )).collect::<Vec<Html>>()}
                    </ul>
                </div>
            </div>
        )
    } else {
        html!(<span>{"error - no combatant found" }</span>)
    }
}
