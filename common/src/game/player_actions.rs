use super::RoguelikeRacerGame;
use crate::app_consts::error_messages;
use crate::character::combatant_properties::CombatantClass;
use crate::errors::AppError;
use crate::game::player_input_handlers::{
    open_treasure_chest, select_consumable, use_selected_consumable,
};
use crate::items::EquipmentSlots;
use crate::packets::client_to_server::{PlayerInputRequest, PlayerInputs};
use serde::{Deserialize, Serialize};

impl RoguelikeRacerGame {
    pub fn process_player_input(
        &mut self,
        player_input_request: PlayerInputRequest,
    ) -> Result<(), AppError> {
        let PlayerInputRequest {
            party_id,
            player_character_id,
            player_input,
        } = player_input_request;

        let adventuring_party =
            self.adventuring_parties
                .get_mut(&party_id)
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::PARTY_NOT_FOUND.to_string(),
                })?;

        let player_character = adventuring_party
            .player_characters
            .get_mut(&player_character_id)
            .ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: "tried to process player input but couldn't find the player character"
                    .to_string(),
            })?;

        match player_input {
            PlayerInputs::SelectConsumable(inventory_slot) => {
                select_consumable::select_consumable(player_character, inventory_slot)?
            }
            PlayerInputs::UseSelectedConsumable => {
                use_selected_consumable::use_selected_consumable(
                    adventuring_party,
                    player_character_id,
                )?
            }
            PlayerInputs::OpenTreasureChest => {
                open_treasure_chest::open_treasure_chest(&mut self.id_generator, adventuring_party)?
            }
            _ => (),
        }

        println!("{:#?}", self);

        Ok(())
    }
}
