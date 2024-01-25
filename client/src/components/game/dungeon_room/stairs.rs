use yew::prelude::*;
use yew::Html;

#[function_component(Stairs)]
pub fn stairs() -> Html {
    html!(
    <div class="h-full w-full flex flex-col justify-center items-center ">
        <h3 class="mb-4" >
            {"You find a staircase leading down..."}
        </h3>
        <img
            class="max-h-full max-w-full"
            src="public/img/stairs-down.png"
            alt="stairs down"
        />
    </div>
    )
}
