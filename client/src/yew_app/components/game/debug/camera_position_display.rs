use crate::comm_channels::CameraPosition;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(CameraPositionDisplay)]
pub fn camera_position_display() -> Html {
    let (bevy_communication_state, _) = use_store::<BevyCommunicationStore>();
    let CameraPosition {
        focus,
        alpha,
        beta,
        radius,
    } = bevy_communication_state.camera_position;

    let alpha = if let Some(alpha) = alpha { alpha } else { 0.0 };
    let beta = if let Some(beta) = beta { beta } else { 0.0 };
    let radius = if let Some(radius) = radius {
        radius
    } else {
        0.0
    };

    html!(
    <div class="border border-slate-400 bg-slate-700 mt-2 w-fit p-2">
        <div>{format!("focus x: {:.2} y: {:.2} z: {:.2}", focus.x, focus.y, focus.z)}</div>
        <div>{format!("alpha: {:.2}", alpha)}</div>
        <div>{format!("beta: {:.2}", beta)}</div>
        <div>{format!("radius: {:.2}", radius)}</div>
    </div>
    )
}
