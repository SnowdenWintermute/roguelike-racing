use std::collections::HashSet;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub players_ready_to_explore: HashSet<String>,
    pub players_ready_to_descend_option: Option<HashSet<String>>,
    pub players: HashSet<String>,
}

#[function_component(PlayersReadyToExplore)]
pub fn players_ready_to_explore(props: &Props) -> Html {
    let mut players_ready_to_descend_list = html!();
    let mut ready_to_explore_waiting_message = AttrValue::from("Wating for players to ready up...");
    let mut ready_to_explore_title = AttrValue::from("Players ready to explore next room");
    if let Some(players_ready_to_descend) = props.players_ready_to_descend_option.clone() {
        ready_to_explore_waiting_message = AttrValue::from("Wating for players to vote...");
        ready_to_explore_title = AttrValue::from("Votes to continue exploring current floor");
        players_ready_to_descend_list = html!(
            <PlayersIncludedInList
                title={AttrValue::from("Votes to descend stairs")}
                waiting_message={AttrValue::from("Wating for players to vote...")}
                players={props.players.clone()}
                list_to_check={players_ready_to_descend}
            />
        );
    }
    html!(
    <div class="flex flex-col flex-grow max-h-full overflow-y-auto">
        <PlayersIncludedInList
            title={ready_to_explore_title}
            waiting_message={ready_to_explore_waiting_message}
            players={props.players.clone()}
            list_to_check={props.players_ready_to_explore.clone()}
        />
        {players_ready_to_descend_list}
    </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct PlayersIncludedInListProps {
    title: AttrValue,
    waiting_message: AttrValue,
    players: HashSet<String>,
    list_to_check: HashSet<String>,
}

#[function_component(PlayersIncludedInList)]
fn players_included_in_list(props: &PlayersIncludedInListProps) -> Html {
    let awaiting_readies = props.list_to_check.len() != props.players.len();

    html!(
        <div class="flex-1 p-2  mb-2 last:mb-0">
            <h3 class="mb-2" >
                {props.title.clone()}
            </h3>
            if awaiting_readies {
                <div>
                    {props.waiting_message.clone()}
                </div>
            }
            {props.players.iter().map(|username| {
                 let is_ready = props.list_to_check.contains(username);
                 let mut ready_class = "";
                 if is_ready {
                     ready_class = "text-green-600"
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
