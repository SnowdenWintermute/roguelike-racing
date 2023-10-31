pub mod available_actions;
pub mod generate_action_menu_items;
use crate::{
    components::game::action_menu::available_actions::GameActions, store::game_store::GameStore,
};
use common::adventuring_party::AdventuringParty;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub adventuring_party: AdventuringParty,
}

#[function_component(ActionMenu)]
pub fn action_menu(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let actions_state = use_state(|| Vec::<GameActions>::new());

    let party = props.adventuring_party.clone();
    use_effect_with((), move |_| {
        let new_actions =
            generate_action_menu_items::generate_action_menu_items(game_state, &party);
        actions_state.set(new_actions);
    });

    html!(
        <section class="w-1/3 max-w-[733px] border border-slate-400 bg-slate-700 mr-4">

        </section>
    )
}
