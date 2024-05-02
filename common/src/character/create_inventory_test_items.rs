use crate::game::RoguelikeRacerGame;
use crate::items::Item;
use crate::items::ItemCategories;

pub fn create_inventory_test_items(game: &mut RoguelikeRacerGame) -> Vec<Item> {
    let mut to_return = vec![];
    // TEST INVENTORY ITEMS
    for _ in 0..=1 {
        let random_consumable =
            Item::generate(&mut game.id_generator, 5, Some(ItemCategories::Consumable));
        to_return.push(random_consumable);
    }
    for _ in 0..10 {
        let random_equipment =
            Item::generate(&mut game.id_generator, 1, Some(ItemCategories::Equipment));
        to_return.push(random_equipment);
    }
    to_return
}
