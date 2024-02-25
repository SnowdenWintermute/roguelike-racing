use crate::adventuring_party::AdventuringParty;
use crate::combatants::combatant_classes::CombatantClass;
use crate::errors::AppError;
use crate::game::getters::get_mut_party;
use crate::game::player::RoguelikeRacerPlayer;
use crate::game::RoguelikeRacerGame;
use crate::monsters::monster_types::MonsterTypes;
use crate::monsters::Monster;
use std::collections::HashMap;

pub fn set_up_test_game() -> Result<(RoguelikeRacerGame, u32, u32, u32), AppError> {
    let mut game = RoguelikeRacerGame::new("test_game".to_string());
    let arbitrary_actor_id = 1;
    let player_name = "test_player".to_string();
    let player = RoguelikeRacerPlayer::new(Some(arbitrary_actor_id), player_name.clone());
    game.players.insert(player_name.clone(), player);
    let party_id = game.id_generator.get_next_entity_id();
    let party = AdventuringParty::new(party_id, "test_party".to_string(), "test_party".to_string());
    game.adventuring_parties.insert(party_id, party);
    game.put_player_in_adventuring_party(party_id, player_name.clone())?;
    let character_id = game.add_character_to_adventuring_party(
        party_id,
        CombatantClass::Warrior,
        "test_character",
        player_name.clone(),
    )?;
    let monster_id = game.id_generator.get_next_entity_id();
    let party = get_mut_party(&mut game, party_id)?;
    let mut test_monster = Monster::new(
        monster_id,
        "test_monster".to_string(),
        MonsterTypes::Scavenger,
    );
    test_monster.combatant_properties.hit_points = 20;

    party.current_room.monsters = Some(HashMap::from([(monster_id, test_monster)]));

    Ok((game, party_id, character_id, monster_id))
}
