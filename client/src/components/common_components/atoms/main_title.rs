use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "text-amber-100".to_string(),
            Color::Ok => "text-green-400".to_string(),
            Color::Error => "text-red-400".to_string(),
        }
    }
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let local_var = "butts";
    props.on_load.emit(format!(
        "loaded with color prop {} and local var {}",
        props.color.to_string(),
        local_var
    ));

    html!(
    <h1 class={format!("font-extrabold {}", props.color.to_string())}>
        {&props.title}
    </h1>
    )
}
