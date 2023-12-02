use super::AdventuringParty;
use crate::{
    dungeon_rooms::{DungeonRoom, DungeonRoomTypes},
    game::id_generator::IdGenerator,
};

impl AdventuringParty {
    pub fn generate_next_room(&mut self, id_generator: &mut IdGenerator) {
        let next_room = DungeonRoom::generate(
            id_generator,
            self.current_floor,
            false,
            Some(DungeonRoomTypes::MonsterLair),
        );
        self.current_room = next_room
    }
}
