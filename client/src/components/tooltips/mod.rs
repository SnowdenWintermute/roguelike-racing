use crate::store::ui_store::UIStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TooltipManager)]
pub fn tooltip_manager() -> Html {
    let (ui_state, _) = use_store::<UIStore>();

    let tooltip_text = if let Some(text) = &ui_state.tooltip_text {
        text.clone()
    } else {
        AttrValue::from("")
    };

    if let Some(tooltip_position) = ui_state.tooltip_position {
        html!(
            <>
            <div class="absolute z-20"
                style={format!("top: {}px; left: {}px;", tooltip_position.1 as i32, tooltip_position.0 as i32)}
            >
                <div class= "border border-slate-400 bg-slate-950 text-zinc-300 p-2 -translate-x-1/2 -translate-y-[100%]">
                    {tooltip_text}
                </div>
            </div>
            </>
        )
    } else {
        html!()
    }
}
