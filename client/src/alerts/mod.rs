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

pub fn set_alert<'a>(alerts: RwSignal<Vec<Alert>>, message: String, last_alert_id: &'a mut u32) {
    alerts.update(move |alert_state| {
        let id = last_alert_id.clone();
        alert_state.push(Alert {
            message,
            alert_type: AlertType::Error,
            id: id.clone(),
        });
        // last_alert_id.update(move |id| *id += 1);
        *last_alert_id += 1;
        set_timeout(
            move || {
                alerts.update(move |alert_state| {
                    remove_alert(alert_state, id.clone());
                });
            },
            std::time::Duration::from_secs(5),
        );
    });
}

pub fn remove_alert(alert_state: &mut Vec<Alert>, id: u32) {
    let mut indices_to_remove = Vec::new();
    for (index, alert) in alert_state.iter().enumerate() {
        if alert.id == id.clone() {
            indices_to_remove.push(index);
            break;
        }
    }

    for index in indices_to_remove {
        alert_state.remove(index);
    }
}
