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
    pub hovered_entity: Option<DetailableEntities>,
    pub selected_item: Option<Item>,
    pub focused_character_id: u32,
    pub viewing_skill_level_up_menu: bool,
    pub viewing_attribute_point_assignment_menu: bool,
    pub viewing_inventory: bool,
    pub selecting_injection_type: bool,
    pub viewing_items_on_ground: bool,
}

pub fn set_item_hovered(game_dispatch: Dispatch<GameStore>, item_option: Option<Item>) {
    game_dispatch.reduce_mut(|store| {
        if let Some(item) = item_option {
            let entity_details = DetailableEntities::Item(item.clone());
            store.hovered_entity = Some(entity_details);
        } else {
            store.hovered_entity = None;
        }
    })
}
