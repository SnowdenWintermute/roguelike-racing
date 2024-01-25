use yew::prelude::*;
use yew::Html;

#[function_component(EmptyRoom)]
pub fn empty_room() -> Html {
    html!(
    <div class="h-full w-full flex flex-col justify-center items-center ">
        <h3 class="mb-4" >
            {"The room is empty except for a door..."}
        </h3>
        <img
            class="max-h-full max-w-full"
            src="public/img/door.png"
            alt="door to next room"
        />
    </div>
    )
}
