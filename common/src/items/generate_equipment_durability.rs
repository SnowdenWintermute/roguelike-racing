use crate::primatives::MaxAndCurrent;
use rand::Rng;

pub fn generate_equipment_durability(max: u8) -> Option<MaxAndCurrent<u8>> {
    let min_starting_durability = 1 + max / 4;
    let max_starting_durability = std::cmp::max(3 * max / 4, 1);
    let current_durability =
        rand::thread_rng().gen_range(min_starting_durability..=max_starting_durability);
    Some(MaxAndCurrent {
        current: current_durability,
        max,
    })
}
