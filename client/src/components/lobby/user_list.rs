use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::lobby_store::LobbyStore;

#[function_component(UserList)]
pub fn user_list() -> Html {
    let (lobby_state, _) = use_store::<LobbyStore>();

    html!(
        <section class="w-[16rem] bg-slate-700 border border-slate-400 p-4">
            <h2 class="text-slate-200 text-l mb-2">
            {"Channel: "} {lobby_state.room.room_name.clone()}
            </h2>
            <ul class="list-none">
                {lobby_state.room.users.iter().map(|username|
                    html!{
                        <li class="h-10 border border-slate-400 flex items-center mb-2">
                            <div class="pl-2 overflow-hidden whitespace-nowrap text-ellipsis">
                                {username.clone()}
                            </div>
                        </li>
                    }).collect::<Html>()
                }
            </ul>
        </section>
    )
}
