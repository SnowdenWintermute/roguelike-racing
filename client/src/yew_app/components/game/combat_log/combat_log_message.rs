use yew::AttrValue;

#[derive(Clone, PartialEq)]
pub enum CombatLogMessageStyle {
    Basic,
    PartyProgress,
    PartyWipe,
    PartyEscape,
    BattleVictory,
}

#[derive(Clone, PartialEq)]
pub struct CombatLogMessage {
    pub message: AttrValue,
    pub style: CombatLogMessageStyle,
    pub timestamp: u64,
}

impl CombatLogMessage {
    pub fn new(message: AttrValue, style: CombatLogMessageStyle, timestamp: u64) -> Self {
        CombatLogMessage {
            message,
            style,
            timestamp,
        }
    }
}
