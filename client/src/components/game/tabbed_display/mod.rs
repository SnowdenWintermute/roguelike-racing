use crate::{
    components::game::{combat_log::CombatLog, combatant_detail_tab::CombatantDetailTab},
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

    html!(
        <section class="flex-grow-[2] border border-slate-400 bg-slate-700">
            {displayed_tab}
        </section>
    )
}
