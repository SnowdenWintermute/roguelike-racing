use super::CombatActionProperties;
use super::CombatActionTarget;
use super::FriendOrFoe;
use super::TargetingScheme;
use crate::app_consts::error_messages;
use crate::errors::AppError;
use crate::game::getters::get_mut_player;
use crate::game::RoguelikeRacerGame;
use strum::IntoEnumIterator;

impl RoguelikeRacerGame {
    pub fn get_action_targets_by_saved_preference_or_default(
        &mut self,
        username: &String,
        combat_action_properties: &CombatActionProperties,
        // ids should come filtered by prohibited combatant states and valid combat action categories
        ally_ids_option: &Option<Vec<u32>>,
        opponent_ids_option: &Option<Vec<u32>>,
    ) -> Result<CombatActionTarget, AppError> {
        let mut new_targets: Option<CombatActionTarget> = None;
        println!("getting targets by username: {username}");

        let player = get_mut_player(self, username)?;
        let target_preferences = &player.target_preferences;
        let targeting_scheme_preference = &target_preferences.targeting_scheme_preference;
        let preferred_category_option = &target_preferences.category;
        let preferred_friendly_single_option = target_preferences.friendly_single;
        let preferred_hostile_single_option = target_preferences.hostile_single;

        // IF SELECTED ACTION CONTAINS PREFERRED TARGETING SCHEME
        if combat_action_properties
            .targeting_schemes
            .contains(&targeting_scheme_preference)
        {
            match targeting_scheme_preference {
                TargetingScheme::Single => {
                    // IF PREFERENCE EXISTS SELECT IT IF VALID
                    if let Some(category) = preferred_category_option {
                        match category {
                            FriendOrFoe::Friendly => {
                                new_targets = get_preferred_or_default_single_target_option(
                                    preferred_friendly_single_option,
                                    ally_ids_option,
                                );
                            }
                            FriendOrFoe::Hostile => {
                                new_targets = get_preferred_or_default_single_target_option(
                                    preferred_hostile_single_option,
                                    opponent_ids_option,
                                );
                            }
                        }
                    }
                    // IF NO VALID PREFERRED SINGLE, GET ANY VALID SINGLE
                    for category in FriendOrFoe::iter().collect::<Vec<FriendOrFoe>>() {
                        if new_targets.is_none() {
                            let ids_option = match category {
                                FriendOrFoe::Friendly => ally_ids_option,
                                FriendOrFoe::Hostile => opponent_ids_option,
                            };
                            if let Some(ids) = ids_option {
                                new_targets = get_preferred_or_default_single_target_option(
                                    ids.first().copied(),
                                    ids_option,
                                );
                            }
                        }
                    }
                }
                TargetingScheme::Area => {
                    if let Some(category) = preferred_category_option {
                        // IF PREFERENCE EXISTS SELECT IT IF VALID
                        new_targets = get_group_targets_option(
                            ally_ids_option,
                            opponent_ids_option,
                            &category,
                        );
                    }
                    // IF NO VALID PREFERRED AREA, GET ANY VALID AREA
                    if new_targets.is_none() {
                        for category in FriendOrFoe::iter().collect::<Vec<FriendOrFoe>>() {
                            if new_targets.is_none() {
                                new_targets = get_group_targets_option(
                                    ally_ids_option,
                                    opponent_ids_option,
                                    &category,
                                );
                            }
                        }
                    }
                }
                TargetingScheme::All => new_targets = Some(CombatActionTarget::All),
            }
        }

        // IF NO VALID TARGET IN PREFERRED SCHEME OR PREFERRED SCHEME NOT VALID GET ANY VALID TARGET
        if new_targets.is_none() {
            for targeting_scheme in combat_action_properties
                .targeting_schemes
                .iter()
                .collect::<Vec<&TargetingScheme>>()
            {
                if new_targets.is_none() {
                    match targeting_scheme {
                        TargetingScheme::Single => {
                            if new_targets.is_none() {
                                for category in FriendOrFoe::iter().collect::<Vec<FriendOrFoe>>() {
                                    if new_targets.is_none() {
                                        let ids_option = match category {
                                            FriendOrFoe::Friendly => ally_ids_option,
                                            FriendOrFoe::Hostile => opponent_ids_option,
                                        };
                                        if let Some(ids) = ids_option {
                                            new_targets =
                                                get_preferred_or_default_single_target_option(
                                                    ids.first().copied(),
                                                    ids_option,
                                                );
                                        }
                                    }
                                }
                            }
                        }
                        TargetingScheme::Area => {
                            if new_targets.is_none() {
                                for category in FriendOrFoe::iter().collect::<Vec<FriendOrFoe>>() {
                                    if new_targets.is_none() {
                                        new_targets = get_group_targets_option(
                                            ally_ids_option,
                                            opponent_ids_option,
                                            &category,
                                        );
                                    }
                                }
                            }
                        }
                        TargetingScheme::All => new_targets = Some(CombatActionTarget::All),
                    }
                }
            }
        }

        new_targets.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::NO_VALID_TARGETS_FOUND.to_string(),
        })
    }
}

fn get_preferred_or_default_single_target_option(
    preferred_single_id_option: Option<u32>,
    ids_to_check_option: &Option<Vec<u32>>,
) -> Option<CombatActionTarget> {
    let mut to_return = None;
    if let Some(preferred_id) = preferred_single_id_option {
        if let Some(ids_to_check) = ids_to_check_option {
            if ids_to_check.contains(&preferred_id) {
                to_return = Some(CombatActionTarget::Single(preferred_id))
            } else if let Some(alternative_id) = ids_to_check.get(0) {
                to_return = Some(CombatActionTarget::Single(*alternative_id))
            }
        }
    }
    to_return
}

fn get_group_target_if_targets_exist(
    ids_option: &Option<Vec<u32>>,
    friend_or_foe: FriendOrFoe,
) -> Option<CombatActionTarget> {
    let mut to_return = None;
    if let Some(ids) = ids_option {
        if ids.len() > 0 {
            to_return = Some(CombatActionTarget::Group(friend_or_foe))
        }
    }
    to_return
}

fn get_group_targets_option(
    ally_ids_option: &Option<Vec<u32>>,
    opponent_ids_option: &Option<Vec<u32>>,
    category: &FriendOrFoe,
) -> Option<CombatActionTarget> {
    match category {
        FriendOrFoe::Friendly => {
            get_group_target_if_targets_exist(ally_ids_option, FriendOrFoe::Friendly)
        }
        FriendOrFoe::Hostile => {
            get_group_target_if_targets_exist(opponent_ids_option, FriendOrFoe::Hostile)
        }
    }
}
