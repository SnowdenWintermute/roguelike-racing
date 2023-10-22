use std::{ops::Deref, rc::Rc};

use gloo::console::log;
use yewdux::prelude::Dispatch;

use crate::store::alert_store::AlertStore;

pub mod alert_manager;

#[derive(Debug, Clone, PartialEq)]
pub enum AlertType {
    Error,
    Success,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    pub message: String,
    pub alert_type: AlertType,
    pub id: u32,
}

pub fn set_alert<'a>(
    alert_state: Rc<AlertStore>,
    alert_dispatch: Dispatch<AlertStore>,
    message: String,
) {
    let dispatch = alert_dispatch.clone();
    dispatch.reduce_mut(|store| {
        let new_alert = Alert {
            message,
            alert_type: AlertType::Error,
            id: store.last_alert_id.clone(),
        };
        store.alerts.push(new_alert);
        store.last_alert_id += 1;
    });
    let id = alert_state.last_alert_id.clone();
    // let deletion_timeout = gloo::timers::callback::Timeout::new(4000, move || {
    //     let dispatch = alert_dispatch.clone();
    //     dispatch.reduce_mut(|store| {
    //         let mut indices_to_remove = Vec::new();
    //         for (index, alert) in alert_state.alerts.iter().enumerate() {
    //             if alert.id == id.clone() {
    //                 indices_to_remove.push(index);
    //                 break;
    //             }
    //         }

    //         for index in indices_to_remove {
    //             store.alerts.remove(index);
    //         }
    //         log!("removed ", store.last_alert_id.clone())
    //     })
    // })
    // .forget();

    //         set_timeout(
    //             move || {
    //                 alerts.update(move |alert_state| {
    //                     remove_alert(alert_state, id.clone());
    //                 });
    //             },
    //             std::time::Duration::from_secs(5),
    //         );
    //     });
}

pub fn remove_alert(alert_state: Rc<AlertStore>, alert_dispatch: Dispatch<AlertStore>, id: u32) {
    let mut indices_to_remove = Vec::new();
    for (index, alert) in alert_state.alerts.iter().enumerate() {
        if alert.id == id.clone() {
            indices_to_remove.push(index);
            break;
        }
    }

    for index in indices_to_remove {
        alert_dispatch.reduce_mut(|store| store.alerts.remove(index));
    }
}
