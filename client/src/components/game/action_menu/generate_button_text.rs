use super::available_actions::GameActions;

pub fn generate_button_text(action: &GameActions) -> &str {
    match action {
        GameActions::ToggleReadyToExplore => "Ready to explore",
        GameActions::SetInventoryOpen(open_status) => {
            if *open_status {
                "Open inventory"
            } else {
                "Close inventory"
            }
        }
        GameActions::ToggleInventoryOpen => "Inventory",
        GameActions::UseAutoinjector => "Use autoinjector",
        GameActions::SelectItem(_id) => "Use Item",
        GameActions::OpenTreasureChest => "Open treasure chest",
        GameActions::TakeItem => "Pick up item",
        GameActions::UseItem => "Use",
        GameActions::DropItem => "Drop",
        GameActions::ShardItem => "Convert to shard",
        GameActions::Attack => "Attack",
        GameActions::UseAbility(_name) => "Use ability",
        GameActions::LevelUpAbility(_name) => "Level up ability",
        GameActions::SetAssignAttributePointsMenuOpen(_open_status) => "Assign attributes",
        GameActions::AssignAttributePoint(_attribute) => "Increase attribute",
    }
}
