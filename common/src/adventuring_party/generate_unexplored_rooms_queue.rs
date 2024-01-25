use super::AdventuringParty;
use crate::dungeon_rooms::DungeonRoomTypes;
use rand::seq::SliceRandom;
use rand::thread_rng;

impl AdventuringParty {
    /// create a list of DungeonRoomTypes to generate when players vote to explore the next room in
    /// their dungeon
    pub fn generate_unexplored_rooms_queue(&mut self) {
        let monster_lairs_per_floor = 1;
        let empty_rooms_per_floor = 1;

        self.unexplored_rooms.clear();
        self.rooms_explored.on_current_floor += 1;

        let mut rooms_vec = vec![];
        for _ in 0..monster_lairs_per_floor {
            rooms_vec.push(DungeonRoomTypes::MonsterLair)
        }
        for _ in 0..empty_rooms_per_floor {
            rooms_vec.push(DungeonRoomTypes::Empty)
        }

        rooms_vec.shuffle(&mut thread_rng());
        for room_type in rooms_vec {
            self.unexplored_rooms.push_back(room_type);
        }

        if self.current_floor == 1 && self.rooms_explored.total == 0 {
            self.unexplored_rooms.push_front(DungeonRoomTypes::Empty);
        }

        self.unexplored_rooms.push_back(DungeonRoomTypes::Stairs);
    }
}
