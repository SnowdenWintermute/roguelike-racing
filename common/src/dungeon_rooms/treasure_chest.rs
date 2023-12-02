use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreasureChest {
    pub is_opened: bool,
    pub is_locked: bool,
    pub is_trapped: bool,
    pub level: u8,
}

impl TreasureChest {
    pub fn generate(level: u8) -> TreasureChest {
        let mut rng = rand::thread_rng();

        let mut is_locked = false;
        if rng.gen_range(1..=5) > 3 {
            is_locked = true;
        }

        let mut is_trapped = false;
        if rng.gen_range(1..=5) > 3 {
            is_trapped = true;
        }

        TreasureChest {
            is_opened: false,
            is_locked,
            is_trapped,
            level,
        }
    }
}
