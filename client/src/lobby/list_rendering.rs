use common::packets::server_to_client::GameListEntry;
use leptos::*;

#[component]
pub fn DynamicList(cx: Scope, initial_length: usize) -> impl IntoView {
    let (game_list, set_game_list) = create_signal(cx, Vec::<GameListEntry>::new());
    let mut next_counter_id = initial_length;
    let initial_games = (0..initial_length)
        .map(|id| {
            (
                id,
                create_signal(
                    cx,
                    GameListEntry {
                        game_name: "test".to_string(),
                        number_of_users: 1,
                        time_started: None,
                    },
                ),
            )
        })
        .collect::<Vec<_>>();

    let (games, set_games) = create_signal(cx, initial_games);

    // create_effect(cx, move |_| {
    set_games.update(move |games| {
        log!("{}", games.len());
        if games.len() < 2 {
            games.push((
                3,
                create_signal(
                    cx,
                    GameListEntry {
                        game_name: "test 2".to_string(),
                        number_of_users: 1,
                        time_started: None,
                    },
                ),
            ));
            log!("pushed now len is {}", games.len());
        }
    });
    // });

    // let add_game = move |_| {
    //     let sig = create_signal(
    //         cx,
    //         GameListEntry {
    //             game_name: "test 2".to_string(),
    //             number_of_users: 1,
    //             time_started: None,
    //         },
    //     );
    //     set_games.update(move |games| games.push((10, sig)));
    //     next_counter_id += 1;
    // };

    view! { cx,
        <div>
            // <button on:click=add_game>
            //     "Add Game"
            // </button>
            <ul>
                <For
                    each=games
                    key=|game| game.0
                    view=move |cx, (id, (game, set_game))| {
                        view! { cx,
                            <li>
                                // <button
                                //     on:click=move |_| set_count.update(|n| *n += 1)
                                // >
                                //     {count}
                                // </button>
                                <button
                                    on:click=move |_| {
                                        set_games.update(|games| {
                                            games.retain(|(game_id, _)| game_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                    {game.get().game_name}
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
