mod get_used_consumable_action_results;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;

impl GameServer {
    pub fn character_uses_selected_consumable_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
    ) -> Result<(), AppError> {
        let action_results = self.get_used_consumable_action_results(actor_id, character_id)?;
        self.handle_new_combat_action_results(actor_id, action_results, character_id)
    }
}
