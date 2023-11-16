mod item_details_tab;
use std::ops::Deref;

use crate::{
    components::game::{
        combat_log::CombatLog, combatant_detail_tab::CombatantDetailTab,
        tabbed_display::item_details_tab::ItemDetailsTab,
    },
    store::{
        game_store::{set_compared_item, DetailableEntities, GameStore},
        ui_store::UIStore,
    },
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TabbedDisplay)]
pub fn tabbed_display() -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (ui_state, _) = use_store::<UIStore>();
    let detailed_entity = &game_state.detailed_entity;
    let hovered_entity = &game_state.hovered_entity;

    let hovered_tab = match hovered_entity {
        Some(detailable) => match detailable {
            DetailableEntities::Combatant(combatant_properties) => {
                Some(html!(<CombatantDetailTab combatant={combatant_properties.clone()}/>))
            }
            DetailableEntities::Item(item) => Some(html!(<ItemDetailsTab item={item.clone()}  />)),
        },
        None => None,
    };

    let detailed_tab = match detailed_entity {
        Some(detailable) => match detailable {
            DetailableEntities::Combatant(combatant_properties) => {
                Some(html!(<CombatantDetailTab combatant={combatant_properties.clone()}/>))
            }
            DetailableEntities::Item(item) => Some(html!(<ItemDetailsTab item={item.clone()} />)),
        },
        None => None,
    };

    let displayed_tab = {
        if let Some(tab) = hovered_tab {
            tab
        } else if let Some(tab) = detailed_tab {
            tab
        } else {
            html!(<CombatLog/>)
        }
    };

    html!(
        <section class="p-2 flex-grow border border-slate-400 bg-slate-700">
            {displayed_tab}
        </section>
    )
}
