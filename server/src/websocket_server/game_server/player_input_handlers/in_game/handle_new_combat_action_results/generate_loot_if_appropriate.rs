use common::game::RoguelikeRacerGame;
use common::items::Item;

pub fn generate_loot(game: &mut RoguelikeRacerGame, num_opponents: u8, dlvl: u8) -> Vec<Item> {
    let mut loot = vec![];
    println!("creating loot for dlvl {dlvl} num num_opponents {num_opponents}");
    for _ in 0..num_opponents {
        loot.push(Item::generate(&mut game.id_generator, dlvl, None))
    }
    println!("generated loot: {:#?}", loot);
    loot
}
