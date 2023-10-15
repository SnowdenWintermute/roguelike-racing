use crate::alerts::Alert;
use leptos::{logging::log, *};

#[component]
pub fn AlertManager() -> impl IntoView {
    let alerts = expect_context::<RwSignal<Vec<Alert>>>();

    view! {
        <div>
            <For
                each=move || alerts.get()
                key=|alert| (alert.message.clone(), alert.id)
                children=|alert| {
                    view! {
                        <div>
                            {alert.message}
                        </div>
                    }
                }
            />
        </div>
    }
}
