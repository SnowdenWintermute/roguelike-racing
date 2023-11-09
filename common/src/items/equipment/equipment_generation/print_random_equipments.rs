use crate::{
    app_consts::DEEPEST_FLOOR,
    game::RoguelikeRacerGame,
    items::{equipment::equipment_generation::name_equipment::name_equipment, Item},
};

pub fn print_random_equipments(game: &mut RoguelikeRacerGame) {
    for i in 1..=DEEPEST_FLOOR {
        for _ in 0..=5 {
            let level = i;
            let item = Item::generate(&mut game.id_generator, level);
            let item_name = match &item.item_properties {
                crate::items::ItemProperties::Consumable(_) => todo!(),
                crate::items::ItemProperties::Equipment(equipment_properties) => {
                    name_equipment(equipment_properties)
                }
            };
            println!("{}\n{}", &item_name, &item);
        }
    }
}
