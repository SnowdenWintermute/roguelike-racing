#![allow(unused)]
pub mod adventuring_party;
pub mod app_consts;
pub mod character;
pub mod combat;
pub mod dungeon_rooms;
pub mod errors;
pub mod game;
pub mod items;
pub mod monster;
pub mod packets;
pub mod primatives;
pub mod status_effects;
pub mod utils;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
