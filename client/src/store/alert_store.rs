use crate::components::alerts::Alert;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Debug)]
pub struct AlertStore {
    pub alerts: Vec<Alert>,
    pub last_alert_id: u32,
}
