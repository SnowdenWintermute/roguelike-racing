pub mod character;
pub mod consts;
pub mod dungeon_rooms;
pub mod equipment;
pub mod game;
pub mod monster;
pub mod primatives;

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
