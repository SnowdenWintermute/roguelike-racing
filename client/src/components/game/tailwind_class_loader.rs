use yew::prelude::*;

#[function_component(TailwindClassLoader)]
pub fn tailwind_class_loader() -> Html {
    html!(
    <>
        <div class="bg-ffxipink" />
        <div class="border-ffxipink" />
        <div class="bg-blue-700" />
        <div class="border-t-yellow-700" />
        <div class="border-blue-700" />
        <div class="border-t-green-600" />
        <div class="border-green-600" />
        <div class="border-green-700" />
    </>
    )
}
