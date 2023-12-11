#[derive(Debug)]
pub struct Character {
    pub speed: u8,
    pub movement: i16,
    pub name: String,
}

impl Character {
    pub fn new(speed: u8, name: String) -> Self {
        Character {
            speed,
            movement: speed as i16,
            name,
        }
    }
}
static TURN_TIME: i16 = 35;
fn main() {
    let mut characters = vec![
        Character::new(40, "A".to_string()),
        Character::new(35, "B".to_string()),
        Character::new(30, "C".to_string()),
    ];

    let mut a_turns = 0;
    let mut b_turns = 0;
    let mut c_turns = 0;

    for _i in 0..20 {
        characters.sort_by(|a, b| b.movement.partial_cmp(&a.movement).unwrap());
        if characters[0].movement >= TURN_TIME {
            println!("{}: {}", characters[0].name, characters[0].movement);
            take_turn(&mut characters[0]);
            if characters[0].name == "A".to_string() {
                a_turns += 1;
            } else if characters[0].name == "B".to_string() {
                b_turns += 1;
            } else {
                c_turns += 1;
            }
        } else {
            for character in characters.iter_mut() {
                character.movement += character.speed as i16;
            }
        }
    }
    println!("a turns: {a_turns} b turns: {b_turns}");
    let total = a_turns + b_turns;
    println!("total: {total}");
    let a_perc = a_turns as f32 / total as f32;
    let b_perc = b_turns as f32 / total as f32;
    let c_perc = c_turns as f32 / total as f32;
    println!(
        "a turns: %{:?} b turns: %{b_perc}, c turns: ${c_perc}",
        a_perc
    );
}

fn take_turn(character: &mut Character) {
    character.movement -= TURN_TIME;
}
