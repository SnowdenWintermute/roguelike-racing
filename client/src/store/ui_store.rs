use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct UIStore {
    pub mod_key_held: bool,
}
