use crate::utils::get_current_time;
use crate::yew_app::components::game::combat_log::combat_log_message::CombatLogMessage;
use crate::yew_app::components::game::combat_log::combat_log_message::CombatLogMessageStyle;
use crate::yew_app::store::game_store::GameStore;
use common::errors::AppError;
use common::packets::GameMessages;
use yew::AttrValue;
use yewdux::Dispatch;

pub fn new_game_message_handler(
    game_dispatch: Dispatch<GameStore>,
    packet: GameMessages,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        let combat_log_message = match packet {
            GameMessages::PartyDescent(party_name, new_floor) => CombatLogMessage::new(
                AttrValue::from(format!("party {party_name} descended to floor {new_floor}")),
                CombatLogMessageStyle::PartyProgress,
                get_current_time() as u64 / 1000,
            ),
            GameMessages::PartyEscape(party_name, time) => CombatLogMessage::new(
                AttrValue::from(format!("party {party_name} escaped the dungeon at {time}")),
                CombatLogMessageStyle::PartyEscape,
                time,
            ),
            GameMessages::PartyWipe(party_name, floor, time) => CombatLogMessage::new(
                AttrValue::from(format!(
                    "party {party_name} wiped on floor {floor} at {time}"
                )),
                CombatLogMessageStyle::PartyWipe,
                time,
            ),
        };
        store.combat_log.push(combat_log_message);
        Ok(())
    })
}
