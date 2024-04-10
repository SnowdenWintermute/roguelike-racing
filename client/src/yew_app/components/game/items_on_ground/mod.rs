use crate::yew_app::components::common_components::atoms::divider::Divider;
use crate::yew_app::components::game::action_menu::PAGE_SIZE;
use crate::yew_app::components::game::tailwind_class_loader::BUTTON_HEIGHT;
use yew::prelude::*;

#[function_component(ItemsOnGround)]
pub fn items_on_ground() -> Html {
    html!(
    <div class="w-full h-full border border-slate-400 bg-slate-700 p-2 pointer-events-auto"
         style={format!("height: {}rem; ", BUTTON_HEIGHT * PAGE_SIZE as f32)}
    >
        {"Items on the ground"}
        <Divider />
    </div>
    )
}
