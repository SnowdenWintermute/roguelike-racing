use rand::seq::SliceRandom;

use crate::random_names::PLAYER_FIRST_NAMES;
use crate::random_names::PLAYER_LAST_NAMES;

pub fn generate_random_username() -> String {
    let mut rng = rand::thread_rng();
    let first_name = PLAYER_FIRST_NAMES.choose(&mut rng).unwrap();
    let last_name = PLAYER_LAST_NAMES.choose(&mut rng).unwrap();
    format!("{first_name} {last_name}").to_string()
}
