use yew::prelude::*;

#[function_component(WelcomeInfo)]
pub fn welcome_info() -> Html {
    html!(
        <section class="flex-1 p-4 mb-4 mr-4 bg-slate-700 border border-slate-400 overflow-y-auto">
            <h3 class="text-lg">{"Roguelike Racing"}</h3>
            <h5 class="mb-2">
            {"alpha 0.2.0"}
            </h5>
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
        </section>
    )
}
