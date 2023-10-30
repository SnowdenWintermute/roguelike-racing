pub mod available_actions;
use crate::{
    components::game::action_menu::available_actions::{GameActions, MenuTypes},
    store::game_store::GameStore,
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
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let actions_state = use_state(|| Vec::<GameActions>::new());

    let party = props.adventuring_party.clone();
    use_effect_with((), move |_| {
        let mut new_menus: Vec<MenuTypes> = Vec::new();
        let mut new_actions: Vec<GameActions> = Vec::new();
        if game_state.viewing_items_on_ground {
            new_menus.push(MenuTypes::ItemsOnGround);
            new_actions = MenuTypes::get_menu(new_menus, None, None, None, None);
        } else if game_state.selected_item.is_some() {
            new_menus.push(MenuTypes::ItemSelected);
            new_actions = MenuTypes::get_menu(new_menus, None, None, None, None);
        } else if game_state.viewing_inventory {
            new_menus.push(MenuTypes::InventoryOpen);
            // new_actions = MenuTypes::get_menu(new_menus, None, None, None, None);
        } else if game_state.viewing_skill_level_up_menu {
            new_menus.push(MenuTypes::LevelUpAbilities)
        } else if game_state.viewing_attribute_point_assignment_menu {
            new_menus.push(MenuTypes::AttributePointAssignment)
        } else if party.current_room.monsters.is_none()
            && !game_state.viewing_inventory
            && !game_state.viewing_items_on_ground
            && game_state.selected_item.is_none()
        {
            new_menus.push(MenuTypes::OutOfCombat);
            if party.current_room.treasure_chest.is_some() {
                new_menus.push(MenuTypes::UnopenedChest);
            }
            if party.current_room.items.is_some() {
                new_menus.push(MenuTypes::ItemsOnGround);
            }
        } else {
            new_menus.push(MenuTypes::InCombat)
        }

        actions_state.set(new_actions);
    });

    html!(
        <section class="w-1/3 max-w-[733px] border border-slate-400 bg-slate-700 mr-4">

        </section>
    )
}
