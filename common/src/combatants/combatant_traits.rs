use serde::Deserialize;
use serde::Serialize;

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum CombatantTraits {
    HpBioavailability,
}
