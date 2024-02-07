use crate::app_consts::MAX_CRIT_CHANCE;
use rand::Rng;

pub fn roll_crit(crit_chance_as_percentage: u16) -> bool {
    let spell_crit_chance = std::cmp::min(MAX_CRIT_CHANCE, crit_chance_as_percentage);
    let mut rng = rand::thread_rng();
    let crit_roll = rng.gen_range(0..=100);
    crit_roll < spell_crit_chance
}
