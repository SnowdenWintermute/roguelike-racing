use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(TargetingIndicator)]
pub fn targeting_indicator(props: &Props) -> Html {
    html!(
        <div class="w-0 h-0 border-t-[1.5rem] border-t-yellow-700
        border-r-[1.5rem] border-r-transparent border-l-[1.5rem] border-l-transparent
        " />
    )
}
