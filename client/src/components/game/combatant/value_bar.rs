use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub max: u16,
    pub curr: u16,
    pub color: AttrValue,
}

#[function_component(ValueBar)]
pub fn value_bar(props: &Props) -> Html {
    let Props { curr, max, color } = props;
    let percent_of_max = if *max > 0 {
        let percise = *curr as f32 / *max as f32 * 100.00;
        // log!(format!(
        //     "percent of max: {percent_of_max} curr: {curr} max: {max} curr/max:{}",
        //     curr / max
        // ));
        percise.round() as u16
    } else {
        100
    };

    let container_styles = format!("relative h-full w-full border border-{color}");
    let inner_bar_styles = format!("h-full bg-{color}");

    html!(
    <div class={container_styles}>
        <div class={inner_bar_styles} style={format!("width: {percent_of_max}%;")} />
        <div class="text-white absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" >{format!("{curr} / {max}")}</div>
    </div>
    )
}
