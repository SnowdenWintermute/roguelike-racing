use yew::prelude::*;

#[function_component(WelcomeInfo)]
pub fn welcome_info() -> Html {
    let show_patch_notes_state = use_state(|| false);
    let cloned_show_patch_notes_state = show_patch_notes_state.clone();
    let handle_show_patch_notes_click = Callback::from(move |_| {
        let new_state = !*cloned_show_patch_notes_state;
        cloned_show_patch_notes_state.set(new_state);
    });
    let patch_notes_button_text = if *show_patch_notes_state {
        "ⓘ  view patch notes"
    } else {
        "ⓘ  view patch notes"
    };

    html!(
        <section class="h-[19rem] max-h-[19rem] p-4 mb-4 mr-4 bg-slate-700 border border-slate-400 overflow-y-auto">
        <div class="flex justify-between mb-2">
            <h3 class="text-lg mb-2">{"Roguelike Racing alpha 0.5.0 "}</h3>
            <button onclick={handle_show_patch_notes_click} class="border border-slate-400 p-[.25rem] pr-2 pl-2">
                {patch_notes_button_text}
            </button>
        </div>
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
    <p class="font-bold" >{"0.5.0 2/15/2024"}</p>
    <p class="underline" >{"Summary:"}</p>
    <p class="mb-2">{ "A large refactor combined the code handling using abilities and consumables into a single generic pipeline, enabling the creation of two new abilities, 'Fire' and 'Healing', and laying the groundwork for creating new actions more easily. Monster variety has been increased, and some quality of life features such as stacking consumables and more intuitive keyboard controls were implemented." }</p>
    <p class="underline" >{"Balance changes:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{ "+HP, +Acc and +MP affixes have had their values increased" }</li>
        <li>{ "Two handed weapons now roll 1.75x more attributes for their affixes than other items" }</li>
        <li>{ "Monster attribute scaling with dungeon level tweaked" }</li>
        <li>{ "Dead characters now return to life with 1 HP after a battle victory" }</li>
    </ul>
    <p class="underline" >{"Fixed bugs:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{ "Autoinjectors are no longer AOE targetable" }</li>
        <li>{ "Vitality now adds HP as intended" }</li>
        <li>{ "Changing the focused character while they have an action selected will now deselect it" }</li>
        <li>{ "Jewelry no longer can roll the Armor Class Percentage affix" }</li>
        <li>{ "Dead characters can no longer take actions in battles they didn't die in" }</li>
        <li>{ "Inventory can now be exited from any page" }</li>
        <li>{ "Dead characters can no longer use abilities outside of combat" }</li>
    </ul>
    <p class="underline" >{"Added features:"}</p>
    <ul class="list-disc list-inside mb-1">
        <li>{ "New ability, 'Fire', a magical spell that deals fire elemental type damage" }</li>
        <li>{ "New ability, 'Healing', a magical spell that restores HP to targets, or if they are 'undead', damages them for 1.5x the rolled value"}</li>
        <li>{ "Intelligence now adds MP and increases spell damage and healing"}</li>
        <li>{ "Focus now adds increases spell critical strike chance and damage, and penetrates Resilience"}</li>
        <li>{ "Resilience now increases healing received from magical sources and reduces damage taken from magical sources"}</li>
        <li>{ "New combatant trait, 'Elemental Affinity', which can cause combatants to take more or less damage from elemental abilities, or even be healed by them" }</li>
        <li>{ "New consumable type, the 'MP Autoinjector'" }</li>
        <li>{ "Consumables now show as stacks instead of distinct items in character inventories" }</li>
        <li>{ "Action menu hotkeys for 'Next' and 'Previous' actions now use dedicated keys 'E' and 'W' with 'ArrowRight' and 'ArrowLeft' as alternatives" }</li>
        <li>{ "Action menu hotkeys for 'Go Back' and 'Cancel' type actions now use dedicated key 'Escape'" }</li>
        <li>{ "Action menu hotkeys for 'Use/Equip' and 'Confirm' type actions now use dedicated key 'R' with 'Enter' as an alternative" }</li>
    </ul>
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
        {"Welcome to the alpha of Roguelike Racing, a multiplayer turn based
            RPG in the spirit of For the King, Diablo and Final Fantasy."}
        </p>
        <p class="mb-2" >{"The dungeon can be explored solo, cooperatively or competitively. To get started:"}</p>
        <ul class="list-disc list-inside mb-2" >
            <li>{"Create or join a game"}</li>
            <li>{"Create a party with one or more players each controlling one or more characters"}</li>
            <li>{"Explore the dungeon and try to reach the lowest floor before the other parties get there"}</li>
        </ul>
        <p>
            <a class="underline" href="https://discord.gg/MyVPQf2Zzm" >{ "Join us on Discord"  }</a>
        </p>
        <p>
            <span>{"Please report issues at "}</span>
            <a class="underline" href={"https://github.com/SnowdenWintermute/roguelike-racing/issues"}>
                {"https://github.com/SnowdenWintermute/roguelike-racing/issues"}
            </a>
        </p>
    </>
    )
}
