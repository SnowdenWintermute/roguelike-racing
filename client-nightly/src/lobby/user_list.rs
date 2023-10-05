use common::packets::server_to_client::RoomState;
use leptos::*;

#[component]
pub fn user_list() -> impl IntoView {
    let room_state = expect_context::<RwSignal<RoomState>>();
    let users = move || room_state.with(|state| state.users.clone());

    view! {
        <section class="w-20 bg-slate-700 border border-slate-400 p-4 mb-4">
            <h3>"Users in channel "{room_state.with(move |room| room.room_name.clone())}</h3>
            <ul class="list-none">
                  <For
                    each=users
                    key=|user| user.clone()
                    children=move |user| {
                  view! {
                   <li>{user}</li>
                  }
            }
              />
            </ul>
        </section>
    }
}
