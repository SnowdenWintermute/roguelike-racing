use std::num::ParseIntError;

use crate::{components::alerts::remove_alert, store::alert_store::AlertStore};
use gloo::console::log;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(AlertManager)]
pub fn alert_manager() -> Html {
    let (alert_state, alert_dispatch) = use_store::<AlertStore>();

    let handle_alert_click = Callback::from(move |e: MouseEvent| {
        alert_dispatch.reduce_mut(|store| {
            let id: Result<u32, ParseIntError> = e
                .target_unchecked_into::<HtmlElement>()
                .id()
                .split("-")
                .collect::<Vec<&str>>()[1]
                .to_string()
                .parse();
            // log!(format!("id: {}", id));
            if let Ok(id) = id {
                log!("removing alert ", id);
                let mut indices_to_remove = Vec::new();
                for (index, alert) in store.alerts.iter().enumerate() {
                    if alert.id == id.clone() {
                        indices_to_remove.push(index);
                        break;
                    }
                }

                for index in indices_to_remove {
                    store.alerts.remove(index);
                }
            }
        })
    });

    let cloned = handle_alert_click.clone();
    let click_handler_emitter = Callback::from(move |e| {
        cloned.emit(e);
    });

    html!(
        <ul class="absolute p-3 list-none">
            {alert_state.alerts.iter().map(|alert|
                html!{
                    <li>
                        <button id={format!("alert-{}",alert.id.to_string())} class="animate-slide-appear-from-left h-10 mb-2 pl-2 pr-2
                        border border-slate-400 bg-slate-700 text-zinc-300"
                        onclick={click_handler_emitter.clone()}
                        >
                            {alert.message.clone()}
                        </button>
                    </li>
            }).collect::<Html>()}
        </ul>
    )
}
