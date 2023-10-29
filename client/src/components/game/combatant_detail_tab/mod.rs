use crate::{
    components::common_components::atoms::button_basic::ButtonBasic,
    store::game_store::{CombatantDetails, GameStore},
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant: CombatantDetails,
}

#[function_component(CombatantDetailTab)]
pub fn combatant_detail_tab(props: &Props) -> Html {
    let (_, game_dispatch) = use_store::<GameStore>();
    let Props { combatant } = props;
    let _combat_attributes = combatant
        .combatant_properties
        .clone()
        .get_total_attributes();

    let close_display = Callback::from(move |_| {
        game_dispatch.reduce_mut(|store| store.detailed_entity = None);
    });

    html!(
        <div>
        <ButtonBasic onclick={close_display} >{"Close"}</ButtonBasic>
            {"Combatant details for entity id: "}{combatant.entity_properties.id}
        </div>
    )
}
