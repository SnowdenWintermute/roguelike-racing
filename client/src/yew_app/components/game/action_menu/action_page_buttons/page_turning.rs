use crate::yew_app::store::game_store::GameStore;
use yewdux::Dispatch;

pub fn next_page(
    game_dispatch: Dispatch<GameStore>,
    current_page_number: u32,
    number_of_pages: usize,
) {
    if current_page_number as usize >= number_of_pages - 1 {
        game_dispatch.reduce_mut(|store| store.action_menu_current_page_number = 0)
    } else {
        game_dispatch
            .reduce_mut(|store| store.action_menu_current_page_number = current_page_number + 1)
    }
}

pub fn prev_page(
    game_dispatch: Dispatch<GameStore>,
    current_page_number: u32,
    number_of_pages: usize,
) {
    if current_page_number as usize == 0 {
        let new_page_number = number_of_pages - 1;
        game_dispatch
            .reduce_mut(|store| store.action_menu_current_page_number = new_page_number as u32)
    } else {
        game_dispatch
            .reduce_mut(|store| store.action_menu_current_page_number = current_page_number - 1)
    }
}
