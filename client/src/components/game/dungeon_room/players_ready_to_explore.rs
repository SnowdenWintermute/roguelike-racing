use std::collections::HashSet;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub players_ready: HashSet<String>,
    pub players: HashSet<String>,
}

#[function_component(PlayersReadyToExplore)]
pub fn players_ready_to_explore(props: &Props) -> Html {
    let awaiting_readies = props.players_ready.len() != props.players.len();
    html!(
        <div class="p-2 border border-slate-400">
            if awaiting_readies {
                <div>
                    {"Waiting for players to ready up"}
                </div>
            }
            {props.players.iter().map(|username| {
                 let is_ready = props.players_ready.contains(username);
                 let mut ready_class = "";
                 if is_ready {
                     ready_class = "text-green-400"
                 }
                 html!(
                    <div class={ready_class}>
                        {username}
                    </div>
                ) }
            ).collect::<Html>()}
        </div>
    )
}
