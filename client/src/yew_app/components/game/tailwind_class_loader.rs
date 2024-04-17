use yew::prelude::*;

pub const SPACING_REM: f32 = 0.875;
pub const SPACING_REM_SMALL: f32 = 0.625;
// pub const SPACING_REM_XS: f32 = 0.5;
// pub const PAPER_DOLL_WIDTH: f32 = 23.75;
// pub const CHARACTER_ATTRIBUTES_WIDTH: f32 = 24.25;
pub const BUTTON_HEIGHT: f32 = 2.5;
pub const BUTTON_HEIGHT_SMALL: f32 = 1.875;
// pub const ACTION_MENU_WIDTH: f32 = 25.0;

#[function_component(TailwindClassLoader)]
pub fn tailwind_class_loader() -> Html {
    html!(
    <>
        <div class="translate-x-[.5rem] translate-y-[.5rem]" />
        <div class="bg-ffxipink" />
        <div class="text-ffxipink" />
        <div class="border-ffxipink" />
        <div class="bg-blue-700" />
        <div class="border-t-yellow-700" />
        <div class="border-blue-700" />
        <div class="border-t-green-600" />
        <div class="border-green-600" />
        <div class="border-green-700" />
        <div class="text-slate-700" />
        <div class="border-lime-500" />
        <div class="bg-emerald-900" />
        <div class="bg-amber-900 opacity-50" />
    </>
    )
}
