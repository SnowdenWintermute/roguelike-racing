use common::{equipment, game};

fn main() {
    let mut game = game::Game::new();
    game.add_player_character("mikey@mikesilverman.net");
    game.add_player_character("lucella@battleschool.io");
    game.add_player_character("gamefull@battleschool.io");
    // let item = equipment::Item::generate(game, 10);
    // println!("{:#?}", item);
    println!("{:#?}", game);
}
