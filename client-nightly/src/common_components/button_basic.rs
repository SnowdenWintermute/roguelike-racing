use leptos::*;

#[component]
pub fn button_basic(
    children: Children,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(default = "")] extra_styles: &'static str,
    #[prop(default = "button")] button_type: &'static str,
) -> impl IntoView {
    view! {
        <button
        class=format!("{} {}","border border-slate-400 h-10 cursor-pointer pr-4 pl-4
        flex justify-center items-center disabled:opacity-50 disabled:cursor-auto", extra_styles)
        disabled=disabled
        type=button_type
        >
            {children()}
        </button>
    }
}
