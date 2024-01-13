use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::game::character_sheet::character_attributes::CharacterAttributes;
use crate::store::game_store::CombatantDetails;
use crate::store::game_store::GameStore;
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

    let close_display = Callback::from(move |_| {
        game_dispatch.reduce_mut(|store| {
            store.detailed_entity = None;
            store.hovered_entity = None
        });
    });

    html!(
        <div class="flex justify-between">
            <CharacterAttributes
                combatant_properties={combatant.combatant_properties.clone()}
                entity_properties={combatant.entity_properties.clone()}
            />
            <ButtonBasic onclick={close_display} >{"Close"}</ButtonBasic>
        </div>
    )
}
