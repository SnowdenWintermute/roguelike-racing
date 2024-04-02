use yew::AttrValue;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct UIStore {
    pub mod_key_held: bool,
    pub tooltip_position: Option<(f64, f64)>,
    pub tooltip_text: Option<AttrValue>,
}
