use crate::alerts::{remove_alert, Alert};
use leptos::{logging::log, *};
use web_sys::MouseEvent;

#[component]
pub fn AlertManager() -> impl IntoView {
    let alerts = expect_context::<RwSignal<Vec<Alert>>>();

    view! {
        <ul class="absolute p-3 list-none">
            <For
                each=move || alerts.get()
                key=|alert| (alert.message.clone(), alert.id)
                children=move |alert| {
                    view! {
                        <li>
                            <button value=alert.id class="animate-slide-appear-from-left h-10 mb-2 pl-2 pr-2
                                border border-slate-400 bg-slate-700 text-zinc-300" on:click=move |e| {
                                    let id = event_target_value(&e).parse::<u32>().unwrap();
                                    alerts.update(|alert_state| {
                                        remove_alert(alert_state, id);
                                    });
                                }>
                                {alert.message}
                            </button>
                        </li>
                    }
                }
            />
        </ul>
    }
}
