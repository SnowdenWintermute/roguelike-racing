use crate::store::lobby_store::LobbyStore;

pub fn handle_user_left_room(lobby_state: &mut LobbyStore, username_leaving: &str) {
    for (index, username) in lobby_state.room.users.clone().iter().enumerate() {
        if username_leaving == username {
            lobby_state.room.users.remove(index);
        }
    }
}
