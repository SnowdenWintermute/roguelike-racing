use crate::combat::combat_actions::FriendOrFoe;
use crate::combat::combat_actions::TargetingScheme;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoguelikeRacerPlayer {
    pub actor_id: Option<u32>,
    pub party_id: Option<u32>,
    pub username: String,
    pub character_ids: Option<HashSet<u32>>,
    pub target_preferences: CombatActionTargetPreferences,
}

impl RoguelikeRacerPlayer {
    pub fn new(actor_id: Option<u32>, username: String) -> Self {
        RoguelikeRacerPlayer {
            actor_id,
            party_id: None,
            username,
            character_ids: None,
            target_preferences: CombatActionTargetPreferences::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatActionTargetPreferences {
    pub friendly_single: Option<u32>,
    pub hostile_single: Option<u32>,
    pub category: Option<FriendOrFoe>,
    pub targeting_scheme_preference: TargetingScheme,
}

impl Default for CombatActionTargetPreferences {
    fn default() -> Self {
        CombatActionTargetPreferences {
            friendly_single: None,
            hostile_single: None,
            category: None,
            targeting_scheme_preference: TargetingScheme::Single,
        }
    }
}
