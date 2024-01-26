use yew::prelude::*;

#[function_component(WelcomeInfo)]
pub fn welcome_info() -> Html {
    let show_patch_notes_state = use_state(|| false);
    let cloned_show_patch_notes_state = show_patch_notes_state.clone();
    let handle_show_patch_notes_click = Callback::from(move |_| {
        let new_state = !*cloned_show_patch_notes_state;
        cloned_show_patch_notes_state.set(new_state);
    });
    html!(
        <section class="flex-1 p-4 mb-4 mr-4 bg-slate-700 border border-slate-400 overflow-y-auto">
        <h3 class="text-lg">{"Roguelike Racing"}</h3>
        <button onclick={handle_show_patch_notes_click} class="mb-2">
            {"alpha 0.2.1 ⓘ"}
        </button>
        if *show_patch_notes_state == false {
            <WelcomeMessage />
        } else {
            <PatchNotes />
        }
        </section>
    )
}

#[function_component(PatchNotes)]
fn welcome_message() -> Html {
    html!(
        <>
    <p>{"0.3.0 1/26/2024"}</p>
    <p>{"Fixed bugs:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Opening the inventory while combat animations were playing would cause the client to desync"}</li>
        <li>{"Focus now shifts to the active character at the beginning of combat if not in the inventory"}</li>
    </ul>
    <p>{"Added features:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Color coded messages now display in the combat log when any party in a game descends to a new floor, escapes the dungeon or wipes"}</li>
        <li>{"Version history now shows dates"}</li>
    </ul>
    <p>{"0.2.1 1/25/2024"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Added patch notes section to welcome info"}</li>
        <li>{"Fixed a bug where the room exploration tracker wouldn't work on any floor except the first"}</li>
    </ul>
    <p>{"0.2.0 1/24/2024"}</p>
    <p>{"Fixed bugs:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Players couldn't attack while wearing a shield"}</li>
        <li>{"Unequipping an item would not change focus to the unequipped item"}</li>
    </ul>
    <p>{"Added features:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Stairs down now appear in the dungeon and players can vote to keep exploring the current floor or descend to the next one"}</li>
        <li>{"An explored rooms tracker now appears as part of the top info bar"}</li>
        <li>{"The top info bar can be re positioned by clicking on it"}</li>
    </ul>
    </>
    )
}

#[function_component(WelcomeMessage)]
fn welcome_message() -> Html {
    html!(
    <>
        <p class="mb-2">
        {"Welcome to the early alpha of Roguelike Racing, a multiplayer turn based
            RPG in the spirit of For the King, Diablo and Final Fantasy. All layout and graphics are placeholders.
            There is a minimum playable game in which you can do the following:"}
        </p>
        <ul class="list-disc list-inside" >
            <li>{"Create and join games which consist of one or more adventuring parties"}</li>
            <li>{"Create a party with one or more players each controlling one or more characters"}</li>
            <li>{"Explore rooms of the dungeon which may contain monsters"}</li>
            <li>{"Battle monsters and receive randomly generated equipment"}</li>
            <li>{"Equip, trade and discard equipment"}</li>
            <li>{"Try to reach the lowest floor of the dungeon by descending the stairs"}</li>
        </ul>
        <p>
            {"a lot is not implemented, balanced or at all good, but if anything is severely broken you can report it at https://github.com/SnowdenWintermute/roguelike-racing/issues"}
        </p>
    </>
    )
}
