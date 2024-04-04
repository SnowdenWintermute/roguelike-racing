pub mod character_attributes;
pub mod item_details_viewer;
mod paper_doll;
use crate::yew_app::components::game::character_sheet::character_attributes::CharacterAttributes;
use crate::yew_app::components::game::character_sheet::paper_doll::PaperDoll;
use crate::yew_app::components::game::tailwind_class_loader::{
    BUTTON_HEIGHT_SMALL, SPACING_REM, SPACING_REM_SMALL,
};
use crate::yew_app::store::game_store::{get_focused_character, GameStore};
use std::collections::HashMap;
use yew::prelude::*;
use yewdux::prelude::use_store;

pub const CHARACTER_SHEET_HEIGHT: f32 = SPACING_REM_SMALL + BUTTON_HEIGHT_SMALL + SPACING_REM;

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

    // let focused_character = party.characters.get(&game_state.focused_character_id);
    let show_character_sheet =
        game_state.viewing_inventory || game_state.viewing_attribute_point_assignment_menu;
    // && focused_character.is_some();
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
                <button
                    class="border border-slate-400 bg-slate-700 w-40 mr-2.5"
                    style={format!("height: {}rem; ", BUTTON_HEIGHT_SMALL)}
                >
                </button>
                <button
                    class="border border-slate-400 bg-slate-700 w-32 mr-2.5"
                    style={format!("height: {}rem; ", BUTTON_HEIGHT_SMALL)}
                >
                </button>
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
