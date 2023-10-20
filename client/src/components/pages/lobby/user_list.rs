use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(UserList)]
pub fn user_list() -> Html {
    html!(
        <section class="w-[16rem] bg-slate-700 border border-slate-400 p-4">
            <h2 class="text-slate-200 text-l mb-2">
            {"Channel: "}
            </h2>
            <ul class="list-none">
                // <For
                //     each=users
                //     key=|user| user.clone()
                //     children=move |user| {
                //         view! {
                //             <li class="h-10 border border-slate-400 flex items-center mb-2">
                //                 <div class="pl-2 overflow-hidden whitespace-nowrap text-ellipsis">
                //                     {user}
                //                 </div>
                //             </li>
                //         }
                //     }
                // />
            </ul>
        </section>
    )
}
