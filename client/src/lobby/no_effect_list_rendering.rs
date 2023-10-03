use leptos::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ClientGameListState {
    pub games: Vec<GameListEntry>,
}

impl ClientGameListState {
    pub fn new() -> Self {
        ClientGameListState { games: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameListEntry {
    pub game_name: String,
    pub number_of_users: u8,
    pub time_started: Option<u64>,
}

#[component]
pub fn game_list(cx: Scope) -> impl IntoView {
    let (game_list_state, set_game_list_state) = create_signal(cx, ClientGameListState::new());
    let (last_game_id, set_last_game_id) = create_signal(cx, 1);
    let game_list = move || game_list_state.get().games;

    create_effect(cx, move |_| log!("game list: {:#?}", game_list()));

    create_effect(cx, move |_| {
        set_game_list_state.update(move |list_state| {
            list_state.games.push(GameListEntry {
                game_name: "game added in effect".to_string(),
                number_of_users: 1,
                time_started: None,
            });
        })
    });

    let add_game = move |_| {
        set_last_game_id.update(|name| {
            *name += 1;
        });
        set_game_list_state.update(move |state| {
            state.games.push(GameListEntry {
                game_name: last_game_id.get().to_string(),
                number_of_users: 1,
                time_started: None,
            })
        })
    };

    view! { cx,
        <section id="game_list" class="flex-1 p-4 bg-slate-700 border border-lime-500">
            <h3>"Games"</h3>
            <ul class="list-none">
            <button on:click=add_game>
                "Add Game"
            </button>
            <For each=game_list
                key=|game| game.game_name.clone()
                view=move |cx,  game| {
                        view! {cx,
                        <li>"game name: "{game.game_name.clone()}</li>
                        }
                    }
                />
            </ul>
        </section>
    }
}
