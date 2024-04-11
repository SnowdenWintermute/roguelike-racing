pub mod character_attributes;
pub mod item_details_viewer;
mod paper_doll;
use crate::yew_app::components::game::character_sheet::character_attributes::CharacterAttributes;
use crate::yew_app::components::game::character_sheet::paper_doll::PaperDoll;
use crate::yew_app::components::game::tailwind_class_loader::BUTTON_HEIGHT_SMALL;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM_SMALL;
use crate::yew_app::store::game_store::get_current_party_option;
use crate::yew_app::store::game_store::get_focused_character;
use crate::yew_app::store::game_store::GameStore;
use common::game::getters::get_character;
use common::packets::CharacterId;
use std::collections::HashMap;
use yew::prelude::*;
use yewdux::prelude::use_store;

// pub const CHARACTER_SHEET_HEIGHT: f32 = SPACING_REM_SMALL + BUTTON_HEIGHT_SMALL + SPACING_REM;

#[function_component(CharacterSheet)]
pub fn character_sheet() -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let character_option = get_focused_character(&game_state).ok();
    let (equipment, combatant_attributes) = match character_option {
        Some(character) => (
            character.combatant_properties.equipment.clone(),
            character.combatant_properties.get_total_attributes(),
        ),
        None => (HashMap::new(), HashMap::new()),
    };

    use_effect_with((), move |_| {
        move || {
            game_dispatch.reduce_mut(|store| {
                store.hovered_entity = None;
            });
        }
    });

    let party_character_ids = if let Some(party) = get_current_party_option(&game_state) {
        party.character_positions.clone()
    } else {
        Vec::new()
    };

    let show_character_sheet =
        game_state.viewing_inventory || game_state.viewing_attribute_point_assignment_menu;

    let conditional_styles = if show_character_sheet {
        ""
    } else {
        "opacity-0 w-0 overflow-hidden"
    };

    html!(
        <section class={format!("{}", conditional_styles)}>
            <ul class="flex list-none pointer-events-auto"
                style={format!("margin-bottom: {}rem; ", SPACING_REM_SMALL)}
                >
                {party_character_ids.iter().map(|id| html!(
                        <CharacterSheetCharacterSelectionButton id={id} />
                            )).collect::<Html>()}
            </ul>
            <div class="border border-slate-400 bg-slate-700 overflow-y-auto flex pointer-events-auto"
                 style={format!("padding: {}rem; ", SPACING_REM)}
            >
                <PaperDoll equipment={equipment} attributes={combatant_attributes} />
                if let Some(character) = character_option {
                    <CharacterAttributes
                        entity_properties={character.entity_properties.clone()}
                        combatant_properties={character.combatant_properties.clone()}
                        show_attribute_assignment_buttons={true}
                    />
                }
            </div>
        </section>
    )
}

#[derive(Properties, PartialEq)]
struct Props {
    id: CharacterId,
}

#[function_component(CharacterSheetCharacterSelectionButton)]
fn character_sheet_character_selection_button(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let game_result = game_state.get_current_game();
    let character_option = {
        let mut to_return = None;
        if let Ok(game) = game_result {
            if let Some(party_id) = game_state.current_party_id {
                if let Ok(character) = get_character(game, party_id, props.id) {
                    to_return = Some(character)
                }
            }
        }
        to_return
    };

    let character_name = match character_option {
        Some(character) => character.entity_properties.name.clone(),
        None => "".to_string(),
    };

    let cloned_game_dispatch = game_dispatch.clone();
    let id = props.id;
    let handle_click = Callback::from(move |_: MouseEvent| {
        cloned_game_dispatch.reduce_mut(|store| store.focused_character_id = id);
    });

    let is_selected_style = if game_state.focused_character_id == props.id {
        "border-yellow-400"
    } else {
        ""
    };

    html!(
        <button
            class={ format!( "border border-slate-400 bg-slate-700 w-40 mr-2.5 {is_selected_style}" )}
            onclick={handle_click}
            style={format!("height: {}rem; ", BUTTON_HEIGHT_SMALL)}
        >
            {character_name}
        </button>
    )
}
