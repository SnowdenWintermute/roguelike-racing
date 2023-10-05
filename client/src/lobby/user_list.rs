use common::packets::server_to_client::RoomState;
use leptos::*;

#[component]
pub fn user_list() -> impl IntoView {
    let room_state = expect_context::<RwSignal<RoomState>>();
    let users = move || room_state.with(|state| state.users.clone());

    view! {
        <section class="w-[16rem] bg-slate-700 border border-slate-400 p-4">
            <h2 class="text-slate-200 text-l mb-2" >"Channel: ["{move || room_state.with(move |room| room.room_name.clone())}"]"</h2>
            <ul class="list-none">
                  <For
                    each=users
                    key=|user| user.clone()
                    children=move |user| {
                  view! {
                   <li class="h-10 border border-slate-400 flex items-center mb-2" >
                       <div class="pl-2 overflow-hidden whitespace-nowrap text-ellipsis">
                           {user}
                       </div>
                   </li>
                  }
            }
              />
            </ul>
        </section>
    }
}
