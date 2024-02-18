#![allow(unused)]
use crate::errors::AppError;
#[cfg(test)]
#[test]
pub fn test_calculate_combat_action_hp_changes() -> Result<(), AppError> {
    use crate::combat::battle::BattleGroup;
    use crate::combat::battle::BattleGroupTypes;
    use crate::combat::combat_actions::CombatAction;
    use crate::combat::combat_actions::CombatActionTarget;
    use crate::combat::ActionResult;
    use crate::combatants::abilities::CombatantAbilityNames;
    use crate::game::getters::get_mut_party;
    use crate::tests::set_up_test_game::set_up_test_game;

    let (mut game, party_id, character_id, monster_id) = set_up_test_game()?;
    let party = get_mut_party(&mut game, party_id)?;
    let test_character = party.characters.get(&character_id).expect("a character");
    let character_id = test_character.entity_properties.id;
    let ally_ids = party.character_positions.clone();
    let group_a = BattleGroup {
        name: format!("{}", party.name).to_string(),
        party_id: party.id,
        combatant_ids: party.character_positions.clone(),
        group_type: BattleGroupTypes::PlayerControlled,
    };
    let mut monster_ids = party.get_monster_ids()?;
    monster_ids.sort();
    let group_b = BattleGroup {
        name: format!("{}-monsters", party.name).to_string(),
        party_id,
        combatant_ids: monster_ids,
        group_type: BattleGroupTypes::ComputerControlled,
    };
    let battle_id = game.initiate_battle(group_a, group_b)?;

    let party = get_mut_party(&mut game, party_id)?;
    let test_monsters = party
        .current_room
        .monsters
        .clone()
        .expect("test monsters to be present");
    let test_monster = test_monsters
        .get(&monster_id)
        .expect("a test monster to be present");
    for _ in 0..100 {
        let combat_action = CombatAction::AbilityUsed(CombatantAbilityNames::AttackMeleeOffhand);
        let targets = CombatActionTarget::Single(test_monster.entity_properties.id);
        let battle_option = game.battles.get(&battle_id);

        let action_result = ActionResult::new(character_id, combat_action.clone(), targets.clone());
        let action_result = game.calculate_combat_action_hp_changes(
            &action_result,
            character_id,
            &targets,
            battle_option,
            ally_ids.clone(),
            &combat_action,
            Some((1, 1.0)),
        )?;
        println!("{:#?}", action_result.hp_changes_by_entity_id);
    }

    Ok(())
}
