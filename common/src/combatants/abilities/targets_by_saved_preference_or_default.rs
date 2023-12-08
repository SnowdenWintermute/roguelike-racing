use super::{
    get_combatant_ability_attributes::{TargetCategories, TargetingScheme},
    AbilityTarget, CombatantAbilityNames, FriendOrFoe,
};
use crate::{
    adventuring_party::AdventuringParty, app_consts::error_messages,
    combatants::AbilityTargetPreferences, errors::AppError,
};

impl CombatantAbilityNames {
    pub fn targets_by_saved_preference_or_default(
        &self,
        character_id: u32,
        target_preferences: &AbilityTargetPreferences,
        party: &AdventuringParty,
    ) -> Result<AbilityTarget, AppError> {
        let ability_attributes = self.get_attributes();
        if ability_attributes
            .targeting_schemes
            .contains(&target_preferences.targeting_scheme_preference)
        {
            targeting_scheme_targets_or_defaults(
                &target_preferences.targeting_scheme_preference,
                &ability_attributes.valid_target_categories,
                target_preferences,
                character_id,
                party,
            )
        } else {
            let default_targeting_scheme = ability_attributes
                .targeting_schemes
                .first()
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::ClientError,
                    message: error_messages::ABILITY_HAS_NO_TARGETING_SCHEME.to_string(),
                })?;
            targeting_scheme_targets_or_defaults(
                &default_targeting_scheme,
                &ability_attributes.valid_target_categories,
                target_preferences,
                character_id,
                party,
            )
        }
    }
}

fn targeting_scheme_targets_or_defaults(
    targeting_scheme: &TargetingScheme,
    valid_target_categories: &TargetCategories,
    target_preferences: &AbilityTargetPreferences,
    character_id: u32,
    party: &AdventuringParty,
) -> Result<AbilityTarget, AppError> {
    match targeting_scheme {
        TargetingScheme::Single => match valid_target_categories {
            TargetCategories::Opponent => preferred_single_target_or_default(
                FriendOrFoe::Hostile,
                &target_preferences,
                &party,
            ),
            TargetCategories::User => Ok(AbilityTarget::Single(character_id)),
            TargetCategories::Friendly => preferred_single_target_or_default(
                FriendOrFoe::Friendly,
                &target_preferences,
                &party,
            ),
            TargetCategories::Any => match target_preferences.category_of_last_single {
                Some(category) => match category {
                    FriendOrFoe::Friendly => preferred_single_target_or_default(
                        FriendOrFoe::Friendly,
                        &target_preferences,
                        &party,
                    ),
                    FriendOrFoe::Hostile => preferred_single_target_or_default(
                        FriendOrFoe::Hostile,
                        &target_preferences,
                        &party,
                    ),
                },
                None => Ok(AbilityTarget::Single(party.get_monster_ids()?[0])),
            },
        },
        TargetingScheme::Area => match valid_target_categories {
            TargetCategories::Opponent => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
            TargetCategories::User => Ok(AbilityTarget::Single(character_id)),
            TargetCategories::Friendly => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
            TargetCategories::Any => match target_preferences.category_of_last_area {
                Some(category) => match category {
                    FriendOrFoe::Friendly => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
                    FriendOrFoe::Hostile => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
                },
                None => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
            },
        },
        TargetingScheme::All => Ok(AbilityTarget::All),
    }
}

fn preferred_single_target_or_default(
    category: FriendOrFoe,
    target_preferences: &AbilityTargetPreferences,
    party: &AdventuringParty,
) -> Result<AbilityTarget, AppError> {
    match category {
        FriendOrFoe::Friendly => {
            let default_ally_id = party.character_positions.first().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::ClientError,
                message: error_messages::NO_CHARACTERS_IN_PARTY.to_string(),
            })?;
            if let Some(previously_targeted_id) = target_preferences.friendly_single {
                if party.character_positions.contains(&previously_targeted_id) {
                    Ok(AbilityTarget::Single(previously_targeted_id))
                } else {
                    Ok(AbilityTarget::Single(*default_ally_id))
                }
            } else {
                Ok(AbilityTarget::Single(*default_ally_id))
            }
        }
        FriendOrFoe::Hostile => {
            let monster_ids = party.get_monster_ids()?;
            let default_monster_id = monster_ids.first().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::ClientError,
                message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
            })?;
            if let Some(previously_targeted_id) = target_preferences.hostile_single {
                if monster_ids.contains(&previously_targeted_id) {
                    Ok(AbilityTarget::Single(previously_targeted_id))
                } else {
                    Ok(AbilityTarget::Single(*default_monster_id))
                }
            } else {
                Ok(AbilityTarget::Single(*default_monster_id))
            }
        }
    }
}

pub fn is_id_of_existing_opponent(party: &AdventuringParty, id: &u32) -> bool {
    match &party.current_room.monsters {
        Some(monsters) => monsters
            .iter()
            .map(|monster| monster.entity_properties.id)
            .collect::<Vec<u32>>()
            .contains(id),
        None => false,
    }
}
