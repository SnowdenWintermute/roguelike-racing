use crate::random_names::PLAYER_FIRST_NAMES;
use crate::random_names::PLAYER_LAST_NAMES;
use crate::random_names::RANDOM_CHARACTER_NAMES_FIRST;
use crate::random_names::RANDOM_GAME_NAMES_FIRST;
use crate::random_names::RANDOM_GAME_NAMES_LAST;
use crate::random_names::RANDOM_PARTY_NAMES;
use rand::seq::SliceRandom;

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

pub fn generate_random_party_name() -> String {
    let mut rng = rand::thread_rng();
    let first_name = RANDOM_PARTY_NAMES.choose(&mut rng).unwrap();
    format!("{first_name}").to_string()
}

pub fn generate_random_character_name() -> String {
    let mut rng = rand::thread_rng();
    let first_name = RANDOM_CHARACTER_NAMES_FIRST.choose(&mut rng).unwrap();
    // let last_name = RANDOM_CHARACTER_NAMES_LAST.choose(&mut rng).unwrap();
    format!("{first_name}").to_string()
}
