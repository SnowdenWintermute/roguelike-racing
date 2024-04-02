use crate::yew_app::store::alert_store::AlertStore;
use yewdux::prelude::Dispatch;
pub mod alert_manager;

#[derive(Debug, Clone, PartialEq)]
pub enum AlertType {
    Error,
    // Success,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    pub message: String,
    pub alert_type: AlertType,
    pub id: u32,
}

pub fn set_alert<'a>(alert_dispatch: Dispatch<AlertStore>, message: String) {
    let dispatch = alert_dispatch.clone();
    let mut id = 0;
    dispatch.reduce_mut(|store| {
        let new_alert = Alert {
            message,
            alert_type: AlertType::Error,
            id: store.last_alert_id.clone(),
        };
        store.alerts.push(new_alert);
        id = store.last_alert_id.clone();
        store.last_alert_id += 1;
    });

    let _deletion_timeout = gloo::timers::callback::Timeout::new(4000, move || {
        dispatch.reduce_mut(|store| {
            remove_alert(store, id);
        })
    })
    .forget();
}

pub fn remove_alert(alert_state: &mut AlertStore, id: u32) {
    let mut indices_to_remove = Vec::new();
    for (index, alert) in alert_state.alerts.iter().enumerate() {
        if alert.id == id.clone() {
            indices_to_remove.push(index);
            break;
        }
    }

    for index in indices_to_remove {
        alert_state.alerts.remove(index);
    }
}
