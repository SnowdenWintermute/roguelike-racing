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
        <section class="h-72 max-h-72 p-4 mb-4 mr-4 bg-slate-700 border border-slate-400 overflow-y-auto">
        <h3 class="text-lg">{"Roguelike Racing"}</h3>
        <button onclick={handle_show_patch_notes_click} class="mb-2">
            {"alpha 0.4.0 ⓘ"}
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
    <p class="font-bold" >{"0.4.0 2/4/2024"}</p>
    <p class="underline" >{"Fixed bugs:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"The info bar now has a proper home and does not block other UI elements"}</li>
        <li>{"Dead combatants no longer attack"}</li>
        <li>{"Maximum UI dimensions set to 1280px x 1080px relative to root element"}</li>
        <li>{"UI elements should no longer resize their neighbors in strange ways"}</li>
        <li>{"Changing focused character with inventory open sets page to 0 to avoid viewing empty pages"}</li>
        <li>{"Focus should properly switch to the active character when the active status was previously held by a monster"}</li>
        <li>{"Equipment properties are now displayed in order of prefix first, suffix second"}</li>
    </ul>
    <p class="underline" >{"Added features:"}</p>
    <ul class="list-disc list-inside mb-2">
        <li>{"A system for using consumable items"}</li>
        <li>{"A system for using actions out of combat"}</li>
        <li>{"New consumable, 'HP Autoinjector'"}</li>
        <li>{"Combatants may now evade attacks based on their evasion vs. the attacker's accuracy"}</li>
        <li>{"Items on the ground may now be hovered to view their details"}</li>
        <li>{"Turn order system rebuilt"}</li>
        <li>{"Page numbers now show in the action menu"}</li>
        <li>{"The action menu is now scrollable with mousewheel action if the element is not overflowing"}</li>
        <li>{"The party may now be defeated if monsters go first in combat and kill the last party member"}</li>
    </ul>
    <p class="font-bold" >{"0.3.0 1/26/2024"}</p>
    <p class="underline" >{"Fixed bugs:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Opening the inventory while combat animations were playing would cause the client to desync"}</li>
        <li>{"Focus now shifts to the active character at the beginning of combat if not in the inventory"}</li>
    </ul>
    <p class="underline" >{"Added features:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Color coded messages now display in the combat log when any party in a game descends to a new floor, escapes the dungeon or wipes"}</li>
        <li>{"Version history now shows dates"}</li>
    </ul>
    <p class="font-bold" >{"0.2.1 1/25/2024"}</p>
    <ul class="list-disc list-inside mb-2">
        <li>{"Added patch notes section to welcome info"}</li>
        <li>{"Fixed a bug where the room exploration tracker wouldn't work on any floor except the first"}</li>
    </ul>
    <p class="font-bold" >{"0.2.0 1/24/2024"}</p>
    <p class="underline" >{"Fixed bugs:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{"Players couldn't attack while wearing a shield"}</li>
        <li>{"Unequipping an item would not change focus to the unequipped item"}</li>
    </ul>
    <p class="underline" >{"Added features:"}</p>
    <ul class="list-disc list-inside mb-2">
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
