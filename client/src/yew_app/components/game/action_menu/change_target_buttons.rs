use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub number_of_pages: usize,
    // pub hidden: bool,
    pub next_prev_buttons: Vec<Html>,
}

#[function_component(ChangeTargetButtons)]
pub fn change_target_buttons(props: &Props) -> Html {
    html!(
    <ul class={ format!( "flex list-none border border-slate-400 bg-slate-700 w-full justify-between items-center" )}>
        {props.next_prev_buttons.clone()}
    </ul>
    )
}
