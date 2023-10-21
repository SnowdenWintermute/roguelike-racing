use common::game::RoguelikeRacerGame;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct GameStore {
    pub game: Option<RoguelikeRacerGame>,
}
