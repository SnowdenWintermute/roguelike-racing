use leptos::*;

#[component]
pub fn button_basic(
    children: Children,
    #[prop(default = Into::into(false), into)] disabled: MaybeSignal<bool>,
    #[prop(default = "")] extra_styles: &'static str,
) -> impl IntoView {
    view! {
        <button
        class=format!("{} {}","border border-sky-500 h-10 cursor-pointer pr-4 pl-4
        flex justify-center items-center disabled:opacity-50 disabled:cursor-auto", extra_styles)
        disabled=disabled
        // prop:disabled=disabled
        >
            {children()}
        </button>
    }
}
