use common::game;
use std::io;

fn main() -> io::Result<()> {
    let mut game = game::Game::new();
    let mut id_generator = game::IdGenerator {
        last_assigned_entity_id: 0,
    };
    let mike_email = "mikey@mikesilverman.net";
    game.add_player_character(
        &mut id_generator,
        mike_email,
        common::character::CharacterClasses::Mage,
    );

    let character = game
        .player_characters
        .get_mut(mike_email)
        .expect("should exist because just inserted");

    let died = false;
    while !died {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim().parse();
        match input {
            Ok(1) => match character.explore_dungeon(&mut id_generator) {
                Ok(()) => (),
                Err(e) => println!("{}", e.message),
            },
            Ok(_) => println!("unhandled input"),
            Err(e) => {
                println!("{}", e)
            }
        }

        // if character.hit_points.current < 1 {
        //     died = true;
        // }
    }
    println!("you died");
    Ok(())
}

// ANY ROOM:
// improve an ability
// shard an item
//
// ANY ROOM W NO MONSTERS:
// explore_dungeon
// use ability
// use item (out of combat usable)
//
//
// ROOM W TREASURE CHEST:
// open
// pick lock
// force lock
// disarm trap
//
// ROOM W MONSTER:
// use ability
// use item (combat usable)
//
// ROOM W ITEM:
// take item
// shard item
