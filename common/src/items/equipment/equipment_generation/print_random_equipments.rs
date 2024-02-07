use crate::app_consts::DEEPEST_FLOOR;
use crate::game::RoguelikeRacerGame;
use crate::items::equipment::equipment_generation::name_equipment::name_equipment;
use crate::items::Item;
use crate::items::ItemCategories;

pub fn print_random_equipments(game: &mut RoguelikeRacerGame) {
    for _i in 1..=10 {
        for j in 1..=DEEPEST_FLOOR {
            let level = j;
            let item = Item::generate(
                &mut game.id_generator,
                level,
                Some(ItemCategories::Equipment),
            );
            let item_name = match &item.item_properties {
                crate::items::ItemProperties::Consumable(_) => todo!(),
                crate::items::ItemProperties::Equipment(equipment_properties) => {
                    name_equipment(equipment_properties)
                }
            };
            println!("{:?}\n{:#?}", &item_name, &item);
        }
    }
}
