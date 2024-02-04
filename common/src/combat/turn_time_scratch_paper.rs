use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;

use rand::Rng;

#[derive(Debug, Clone)]
struct Combatant {
    speed: u16,
    movement: u16,
    num_turns: u16,
    id: u8,
}

const REQUIRED_MOVEMENT_TO_MOVE: u16 = 999;
const MAX_TICKS_TO_FILL_MOVEMENT: u16 = 10;
const MIN_TICKS_TO_FILL_MOVEMENT: u16 = 5;
const MIN_MOVEMENT_PER_TICK: u16 = REQUIRED_MOVEMENT_TO_MOVE / MAX_TICKS_TO_FILL_MOVEMENT;
const MAX_MOVEMENT_PER_TICK: u16 = REQUIRED_MOVEMENT_TO_MOVE / MIN_TICKS_TO_FILL_MOVEMENT;
const MOVEMENT_RANGE: u16 = MAX_MOVEMENT_PER_TICK - MIN_MOVEMENT_PER_TICK;
const MAX_SPEED: u16 = MIN_MOVEMENT_PER_TICK * 10;
const MIN_SPEED: u16 = MIN_MOVEMENT_PER_TICK * 10 / REQUIRED_MOVEMENT_TO_MOVE * 10;
const SPEED_RANGE: u16 = MAX_SPEED - MIN_SPEED;
// (((OldValue - OldMin) * NewRange) / OldRange) + NewMin

impl Combatant {
    fn tick_movement(&mut self) {
        let adjusted_speed = self.speed * 10;
        let movement_to_add =
            (((adjusted_speed - MIN_SPEED) * MOVEMENT_RANGE) / SPEED_RANGE) + MIN_MOVEMENT_PER_TICK;
        // println!("{} added {} movement", self.id, movement_to_add);
        self.movement += movement_to_add;
        // println!("{:?}", self.movement)
    }

    fn new(speed: u16, id: u8) -> Self {
        let mut rng = rand::thread_rng();

        Combatant {
            speed,
            movement: 0,
            num_turns: 0,
            id, // id: rng.gen::<u8>(),
        }
    }
}

pub struct IdGenerator {
    pub last_generated: u8,
}

impl IdGenerator {
    fn get_next_id(&mut self) -> u8 {
        let next_id = self.last_generated + 1;
        self.last_generated = next_id;
        next_id
    }
}

fn main() {
    let mut id_generator = IdGenerator { last_generated: 0 };
    let mut combatants = vec![
        Combatant::new(0, id_generator.get_next_id()),
        Combatant::new(1, id_generator.get_next_id()),
        Combatant::new(1, id_generator.get_next_id()),
        Combatant::new(1, id_generator.get_next_id()),
        Combatant::new(1, id_generator.get_next_id()),
        Combatant::new(3, id_generator.get_next_id()),
        Combatant::new(2, id_generator.get_next_id()),
        Combatant::new(5, id_generator.get_next_id()),
        Combatant::new(8, id_generator.get_next_id()),
        Combatant::new(12, id_generator.get_next_id()),
        Combatant::new(15, id_generator.get_next_id()),
        // Combatant::new(20, id_generator.get_next_id()),
        // Combatant::new(25, id_generator.get_next_id()),
        // Combatant::new(50, id_generator.get_next_id()),
        // Combatant::new(100, id_generator.get_next_id()),
        Combatant::new(120, id_generator.get_next_id()),
    ];

    let mut num_turns_in_battle_remaining = 180;

    while num_turns_in_battle_remaining > 0 {
        combatants.sort_by_key(|item| Reverse((item.movement, item.id)));
        // let first_combatant_movement = combatants[0].movement;
        // let second_combatant_movement = combatants[1].movement;
        // let next_combatant_equal_movement = first_combatant_movement == second_combatant_movement;
        let first_combatant = &mut combatants[0];
        // println!("first combatant movement: {}", first_combatant.movement);

        if first_combatant.movement >= REQUIRED_MOVEMENT_TO_MOVE {
            println!(
                "{} with speed {} moving with movement {}",
                first_combatant.id, first_combatant.speed, first_combatant.movement
            );
            first_combatant.movement -= REQUIRED_MOVEMENT_TO_MOVE;
            first_combatant.num_turns += 1;
            num_turns_in_battle_remaining -= 1;
        }

        for combatant in &mut combatants {
            if combatant.speed > 0 {
                combatant.tick_movement();
            }
        }
    }

    combatants.sort_by(|a, b| b.num_turns.partial_cmp(&a.num_turns).unwrap());
    println!("{:#?}", combatants);
}

