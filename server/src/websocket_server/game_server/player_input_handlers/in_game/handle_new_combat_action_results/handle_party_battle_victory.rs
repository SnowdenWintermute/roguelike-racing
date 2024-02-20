use crate::websocket_server::game_server::GameServer;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::player_input_handlers::in_game::handle_new_combat_action_results::generate_loot_if_appropriate::generate_loot;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::BattleConclusion;
use common::packets::server_to_client::BattleEndReportPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

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
        let loot = generate_loot(game, num_opponents, dlvl);

        let party = get_mut_party(game, party_id)?;
        // make sure all clients receive the item's existance or else one client can take
        // the item before another client sees it leading to desync
        for item in &loot {
            party
                .items_on_ground_not_yet_received_by_all_clients
                .insert(item.entity_properties.id, vec![]);
        }

        // REMOVE THE BATTLE
        println!("all opponents defeated, concluding battle as victory");
        party.battle_id = None;

        // get list of defeated monsters and their levels
        let mut defeated_monster_levels = Vec::new();
        if let Some(monsters) = party.current_room.monsters {
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

        // @TODO - remove once revives are in the game
        for (_, character) in party.characters.iter_mut() {
            // ADD EXP
            if character.combatant_properties > 0 {
                let mut total_xp_to_award = 0;
                for monster_level in defeated_monster_levels {
                    let base_xp = 30 / num_characters_alive;
                    let level_diff = character.combatant_properties.level - monster_level;
                    let base_xp_diff_multiplier: f32 = 0.25;
                    let diff_multiplier = base_xp_diff_multiplier * level_diff.abs();
                    let mut xp_to_award_for_this_monster = base_xp;

                    if level_diff > 0 {
                        xp_to_award_for_this_monster -= base_xp * diff_multiplier;
                    } else if level_diff < 0 {
                        xp_to_award_for_this_monster += base_xp * diff_multiplier;
                    }
                    total_xp_to_award += xp_to_award_for_this_monster;
                }

                character.combatant_properties.experience_points.current += total_xp_to_award;
                let mut calculating_new_levelups = true;
                while calculating_new_levelups {
                    if let Some(required_to_level) = character
                        .combatant_properties
                        .experience_points
                        .required_for_next_level
                    {
                        if character.combatant_properties.experience_points.current
                            >= required_to_level
                        {
                            character.combatant_properties.level += 1;
                            character.combatant_properties.experience_points -= required_to_level;
                            character
                                .combatant_properties
                                .experience_points
                                .required_for_next_level = Some(required_to_level + 25);
                        } else {
                            calculating_new_levelups = false;
                        }
                    } else {
                        calculating_new_levelups = false;
                    }
                }
            }

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
            }),
            None,
        )
    }
}
