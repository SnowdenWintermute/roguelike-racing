pub mod alert_manager;
use leptos::*;

#[derive(Debug, Clone)]
pub enum AlertType {
    Error,
    Success,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub message: String,
    pub alert_type: AlertType,
    pub id: u32,
}

pub fn set_alert(alerts: RwSignal<Vec<Alert>>, message: String, last_alert_id: RwSignal<u32>) {
    alerts.update(move |alert_state| {
        let id = last_alert_id();
        alert_state.push(Alert {
            message,
            alert_type: AlertType::Error,
            id,
        });
        last_alert_id.update(move |id| *id += 1);
        set_timeout(
            move || {
                alerts.update(move |alert_state| {
                    let mut indices_to_remove = Vec::new();
                    for (index, alert) in alert_state.iter().enumerate() {
                        if alert.id == id {
                            indices_to_remove.push(index);
                        }
                    }

                    for index in indices_to_remove {
                        alert_state.remove(index);
                    }
                });
            },
            std::time::Duration::from_secs(5),
        );
    });
}
