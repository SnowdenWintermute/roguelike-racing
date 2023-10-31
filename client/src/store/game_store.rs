use common::{
    combatants::CombatantProperties, game::RoguelikeRacerGame, items::Item,
    primatives::EntityProperties,
};
use yewdux::prelude::*;

#[derive(PartialEq, Clone)]
pub struct CombatantDetails {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
}

#[derive(PartialEq, Clone)]
pub enum DetailableEntities {
    Combatant(CombatantDetails),
    Item(Item),
}

#[derive(Store, Default, PartialEq, Clone)]
pub struct GameStore {
    pub game: Option<RoguelikeRacerGame>,
    pub current_party_id: Option<u32>,
    pub detailed_entity: Option<DetailableEntities>,
    pub focused_character_id: u32,
    pub viewing_skill_level_up_menu: bool,
    pub viewing_attribute_point_assignment_menu: bool,
    pub viewing_inventory: bool,
    pub selecting_injection_type: bool,
    pub viewing_items_on_ground: bool,
    pub selected_item: Option<Item>,
}
