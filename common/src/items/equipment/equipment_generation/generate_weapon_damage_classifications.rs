use crate::combat::hp_change_source_types::HpChangeSource;
use rand::Rng;

pub fn generate_weapon_damage_classifications(
    possible_classifications: &Vec<HpChangeSource>,
    num_classifications_to_select: u8,
) -> Vec<HpChangeSource> {
    let mut remaining_classifications = possible_classifications.clone();
    let mut classifications_to_return: Vec<HpChangeSource> = Vec::new();
    if num_classifications_to_select as usize > possible_classifications.len() {
        panic!("when generating damage classifications for a weapon, the provided number off classifications to generate was greater than the total number of possible classifications")
    }
    while classifications_to_return.len() < num_classifications_to_select as usize
        && remaining_classifications.len() > 0
    {
        let index = rand::thread_rng().gen_range(0..remaining_classifications.len());
        let classification = remaining_classifications.remove(index);
        classifications_to_return.push(classification)
    }

    classifications_to_return
}
