use crate::combatants::combat_attributes::CombatAttributes;
use rand::Rng;
use std::cmp;
use std::collections::HashMap;

pub fn physical_attack_evaded(
    accuracy: u16,
    target_total_attributes: &HashMap<CombatAttributes, u16>,
) -> bool {
    let target_evasion = target_total_attributes
        .get(&CombatAttributes::Evasion)
        .unwrap_or_else(|| &0);
    let acc_eva_compared = accuracy as i16 - *target_evasion as i16;
    let chance_to_hit = if acc_eva_compared < 5 {
        5
    } else {
        cmp::min(95, acc_eva_compared)
    };

    let mut rng = rand::thread_rng();
    let evaded = rng.gen_range(1..=100) > chance_to_hit;
    evaded
}
