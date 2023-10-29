use crate::store::game_store::CombatantDetails;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant: CombatantDetails,
}

#[function_component(CombatantDetailTab)]
pub fn combatant_detail_tab(props: &Props) -> Html {
    let Props { combatant } = props;

    html!(
        <section class="flex-grow-[2] border border-slate-400 bg-slate-700">
            {"Combatant details for entity id: "}{combatant.entity_properties.id}
        </section>
    )
}
