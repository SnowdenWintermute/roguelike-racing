use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::websocket_store::WebsocketStore;
use common::packets::client_to_server::PlayerInputs;
use common::packets::server_to_client::GameListEntry;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameListItemProps {
    pub game: GameListEntry,
}

#[function_component(GameListItem)]
pub fn game_list_item(props: &GameListItemProps) -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();

    let input = PlayerInputs::JoinGame(props.game.game_name.clone());
    let join_game = Callback::from(move |_| {
        send_client_input(&websocket_state.websocket, input.clone());
    });

    html!(
        <li class="w-full flex border border-slate-400 mb-4 justify-between">
            <div class="flex">
                <div class="h-10 flex items-center w-40 border-r border-slate-400 pl-4">
                    <div class="overflow-hidden whitespace-nowrap overflow-ellipsis">
                        {props.game.game_name.clone()}
                    </div>
                </div>
                <div class="h-10 flex items-center w-24 border-r border-slate-400 pl-4">
                    <div class="overflow-hidden whitespace-nowrap overflow-ellipsis">
                        {"Players:"} {props.game.number_of_users}
                    </div>
                </div>
            </div>
            <ButtonBasic
                onclick={join_game}
                disabled={props.game.time_started.is_some()} extra_styles="border-r-0 border-t-0 border-b-0" >
                {"Join"}
            </ButtonBasic>
        </li>
    )
}
