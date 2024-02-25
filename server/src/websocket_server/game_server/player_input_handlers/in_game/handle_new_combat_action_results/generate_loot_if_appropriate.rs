use common::game::RoguelikeRacerGame;
use common::items::Item;
use common::utils::server_log;

pub fn generate_loot(game: &mut RoguelikeRacerGame, num_opponents: u8, dlvl: u8) -> Vec<Item> {
    let mut loot = vec![];
    server_log(&format!(
        "creating loot for dlvl {dlvl} ({num_opponents} items)"
    ));
    for _ in 0..num_opponents {
        loot.push(Item::generate(&mut game.id_generator, dlvl, None))
    }
    loot
}
