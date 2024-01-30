use rand::seq::SliceRandom;

use crate::random_names::PLAYER_FIRST_NAMES;
use crate::random_names::PLAYER_LAST_NAMES;
use crate::random_names::RANDOM_GAME_NAMES_FIRST;
use crate::random_names::RANDOM_GAME_NAMES_LAST;

pub fn generate_random_username() -> String {
    let mut rng = rand::thread_rng();
    let first_name = PLAYER_FIRST_NAMES.choose(&mut rng).unwrap();
    let last_name = PLAYER_LAST_NAMES.choose(&mut rng).unwrap();
    format!("{first_name} {last_name}").to_string()
}

pub fn generate_random_game_name() -> String {
    let mut rng = rand::thread_rng();
    let first_name = RANDOM_GAME_NAMES_FIRST.choose(&mut rng).unwrap();
    let last_name = RANDOM_GAME_NAMES_LAST.choose(&mut rng).unwrap();
    format!("{first_name} {last_name}").to_string()
}
