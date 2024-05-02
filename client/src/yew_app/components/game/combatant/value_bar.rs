use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub max: u16,
    pub curr: u16,
    pub color: AttrValue,
    #[prop_or(false)]
    pub hide_numbers: bool,
}

#[function_component(ValueBar)]
pub fn value_bar(props: &Props) -> Html {
    let Props {
        curr,
        max,
        color,
        hide_numbers,
    } = props;
    let percent_of_max = if *max > 0 {
        let percise = *curr as f32 / *max as f32 * 100.00;
        percise.round() as u16
    } else {
        0
    };

    let container_styles = format!("relative h-full w-full border border-{color}");
    let inner_bar_styles = format!("h-full bg-{color}");

    html!(
    <div class={container_styles}>
        <div class={inner_bar_styles} style={format!("width: {percent_of_max}%;")} />
        if !hide_numbers {
            <div class="text-xs text-white absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" >{format!("{curr} / {max}")}</div>
        }
    </div>
    )
}
