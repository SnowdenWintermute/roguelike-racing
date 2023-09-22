use common::{character::abilities::CharacterAbilities, game, status_effects::StatusEffects};
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
        common::character::combatant_properties::CombatantClass::Mage,
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
            Ok(2) => {
                if character
                    .combatant_properties
                    .status_effects
                    .contains(&StatusEffects::Slowed)
                {
                    // do monster action first
                }
                match character.combatant_properties.perform_combat_action(
                    character.entity_properties.id,
                    character
                        .current_room
                        .monster
                        .as_ref()
                        .unwrap()
                        .entity_properties
                        .id,
                    common::combat::CombatAction::UseCombatantAbility(
                        character
                            .combatant_properties
                            .abilities
                            .get(&CharacterAbilities::Attack)
                            .expect("all characters have attack ability"),
                    ),
                    character
                        .current_room
                        .monster
                        .expect("should be a monster")
                        .combatant_properties,
                ) {
                    Ok(combat_events) => {
                        println!("{:#?}", combat_events);
                        // process combat events
                        // if monster didn't already go, get and process their actions
                        let monster_combat_events = character
                            .current_room
                            .monster
                            .unwrap()
                            .combatant_properties
                            .perform_combat_action(
                                character.current_room.monster.unwrap().entity_properties.id,
                                character.entity_properties.id,
                                common::combat::CombatAction::UseCombatantAbility(
                                    character
                                        .current_room
                                        .monster
                                        .unwrap()
                                        .combatant_properties
                                        .abilities
                                        .get(&CharacterAbilities::Attack)
                                        .expect("all characters have attack ability"),
                                ),
                                character.combatant_properties,
                            );
                    }
                    Err(e) => println!("{}", e.message),
                }
            }
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
