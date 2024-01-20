use crate::store::game_store::GameStore;
use common::errors::AppError;
use common::packets::server_to_client::BattleEndReportPacket;
use yewdux::Dispatch;

pub fn handle_battle_end_report(
    game_dispatch: Dispatch<GameStore>,
    packet: BattleEndReportPacket,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| store.current_battle_end_report = Some(packet));
    Ok(())
}
