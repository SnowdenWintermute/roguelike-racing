use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdGenerator {
    pub last_assigned_entity_id: u32,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator {
            last_assigned_entity_id: 0,
        }
    }
    pub fn get_next_entity_id(&mut self) -> u32 {
        self.last_assigned_entity_id += 1;
        self.last_assigned_entity_id
    }
}
