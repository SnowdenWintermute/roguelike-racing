use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from(""))]
    pub styles: AttrValue,
}

#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
    html!(
    <div id="divider" class={format!("bg-slate-400 h-[1px] flex mt-2 mb-2 {}", &props.styles)}/>
    )
}
