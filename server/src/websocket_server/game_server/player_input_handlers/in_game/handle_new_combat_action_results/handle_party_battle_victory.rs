use crate::websocket_server::game_server::GameServer;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::player_input_handlers::in_game::handle_new_combat_action_results::award_experience_points::award_experience_points;
use crate::websocket_server::game_server::player_input_handlers::in_game::handle_new_combat_action_results::generate_loot_if_appropriate::generate_loot;
use common::combatants::award_levelups::award_levelups;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::ExperienceChange;
use common::packets::server_to_client::BattleConclusion;
use common::packets::server_to_client::BattleEndReportPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use common::utils::server_log;

impl GameServer {
    pub fn handle_party_battle_victory(
        &mut self,
        actor_id: u32,
        battle_id: u32,
        num_opponents: u8,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData { game, party_id, .. } =
            get_mut_game_data_from_actor_id(self, actor_id)?;
        let party = get_mut_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();
        let dlvl = party.current_floor;
        let loot = generate_loot(game, num_opponents + 1, dlvl);

        let party = get_mut_party(game, party_id)?;
        // make sure all clients receive the item's existance or else one client can take
        // the item before another client sees it leading to desync
        for item in &loot {
            party
                .items_on_ground_not_yet_received_by_all_clients
                .insert(item.entity_properties.id, vec![]);
        }

        // REMOVE THE BATTLE
        server_log(&format!(
            "all opponents defeated, concluding battle as victory for party {}",
            party.name
        ));
        party.battle_id = None;

        // get list of defeated monsters and their levels
        let mut defeated_monster_levels = Vec::new();
        if let Some(monsters) = &party.current_room.monsters {
            for (_, monster) in monsters {
                defeated_monster_levels.push(monster.combatant_properties.level)
            }
        }

        party.current_room.monsters = None;
        party.current_room.items.append(&mut loot.clone());

        let mut num_characters_alive = 0;
        for (_, character) in party.characters.iter() {
            if character.combatant_properties.hit_points > 0 {
                num_characters_alive += 1;
            }
        }

        let mut experience_change_reports = Vec::new();
        for (_, character) in party.characters.iter_mut() {
            // ADD EXP
            if character.combatant_properties.hit_points > 0 {
                let total_exp_awarded = award_experience_points(
                    &mut character.combatant_properties,
                    &defeated_monster_levels,
                    num_characters_alive,
                );
                experience_change_reports.push(ExperienceChange {
                    combatant_id: character.entity_properties.id,
                    experience_change: total_exp_awarded as i16,
                });

                award_levelups(&mut character.combatant_properties);
            }

            // @TODO - remove once revives are in the game
            if character.combatant_properties.hit_points == 0 {
                character.combatant_properties.hit_points = 1;
            }
        }

        game.battles.remove(&battle_id);

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::BattleEndReport(BattleEndReportPacket {
                conclusion: BattleConclusion::Victory,
                loot: Some(loot),
                exp_changes: Some(experience_change_reports),
            }),
            None,
        )
    }
}
