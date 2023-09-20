pub mod character;
pub mod combat;
pub mod consts;
pub mod dungeon_rooms;
pub mod game;
pub mod items;
pub mod monster;
pub mod primatives;
pub mod status_effects;

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
