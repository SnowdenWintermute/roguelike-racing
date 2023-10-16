use common::{
    game::{RoguelikeRacerGame, RoguelikeRacerPlayer},
    packets::server_to_client::RoomState,
};
use leptos::*;

pub fn handle_user_left_room(room: RwSignal<RoomState>, username_leaving: &str) {
    room.update(move |room_state| {
        for (index, username) in room_state.users.clone().iter().enumerate() {
            if username_leaving == username {
                room_state.users.remove(index);
            }
        }
    })
}

pub fn handle_user_joined_game(game: RwSignal<Option<RoguelikeRacerGame>>, username: String) {
    game.update(move |game_option| {
        if let Some(game) = game_option {
            game.players
                .insert(username.clone(), RoguelikeRacerPlayer::new(None, username));
        }
    })
}

pub fn handle_user_left_game(game: RwSignal<Option<RoguelikeRacerGame>>, username: String) {
    game.update(move |game_option| {
        if let Some(game) = game_option {
            let _ = game.remove_player_from_adventuring_party(username.clone());
            game.players.remove(&username);
        }
    })
}
