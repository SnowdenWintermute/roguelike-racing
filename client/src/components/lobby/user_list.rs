use crate::store::websocket_store::WebsocketStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(UserList)]
pub fn user_list() -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let lobby_channel = websocket_state.websocket_channels.main;

    html!(
        <section class="w-[16rem] bg-slate-700 border border-slate-400 p-4">
            <h2 class="text-slate-200 text-l mb-2">
                {"Channel: "} {lobby_channel.name.clone()}
            </h2>
            <ul class="list-none">
                {lobby_channel.usernames_in_channel.iter().map(|username|
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
