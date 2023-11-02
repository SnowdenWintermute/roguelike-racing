mod item_details_tab;
use crate::{
    components::game::{
        combat_log::CombatLog, combatant_detail_tab::CombatantDetailTab,
        tabbed_display::item_details_tab::ItemDetailsTab,
    },
    store::game_store::{DetailableEntities, GameStore},
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TabbedDisplay)]
pub fn tabbed_display() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let mut displayed_tab = html!(<CombatLog />);

    if let Some(detailed_entity) = &game_state.detailed_entity {
        match detailed_entity {
            DetailableEntities::Combatant(combatant_details) => {
                displayed_tab = html!(<CombatantDetailTab combatant={combatant_details.clone()} />);
            }
            DetailableEntities::Item(_item) => {
                displayed_tab = html!(<div>{"item: "}</div>);
            }
        }
    }

    if let Some(hovered_entity) = &game_state.hovered_entity {
        match hovered_entity {
            DetailableEntities::Combatant(combatant_details) => {
                displayed_tab = html!(<CombatantDetailTab combatant={combatant_details.clone()} />);
            }
            DetailableEntities::Item(item) => {
                let cloned_item = item.clone();
                displayed_tab = html!(<ItemDetailsTab item={cloned_item} />);
            }
        }
    }

    html!(
        <section class="p-2 flex-grow border border-slate-400 bg-slate-700">
            {displayed_tab}
        </section>
    )
}
