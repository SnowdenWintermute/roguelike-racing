use crate::store::ui_store::UIStore;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::use_store;
use yewdux::Dispatch;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tooltip_text: AttrValue,
    pub children: Html,
}

#[function_component(HoverableTooltipWrapper)]
pub fn hoverable_tooltip_wrapper(props: &Props) -> Html {
    let (_, ui_dispatch) = use_store::<UIStore>();
    let element_ref = use_node_ref();

    let show_tooltip = move |ui_dispatch: Dispatch<UIStore>,
                             element_option: Option<HtmlElement>,
                             text: AttrValue| {
        if let Some(element) = element_option {
            let element_x = element.get_bounding_client_rect().x();
            let element_y = element.get_bounding_client_rect().y();
            let element_width = element.get_bounding_client_rect().width();
            ui_dispatch.reduce_mut(|store| {
                store.tooltip_position = Some((element_x + element_width / 2.0, element_y));
                store.tooltip_text = Some(text);
            });
        }
    };
    let hide_tooltip = move |ui_dispatch: Dispatch<UIStore>| {
        ui_dispatch.reduce_mut(|store| {
            store.tooltip_position = None;
            store.tooltip_text = None;
        })
    };

    let cloned_node_ref = element_ref.clone();
    let cloned_ui_dispatch = ui_dispatch.clone();
    let cloned_text = props.tooltip_text.clone();
    let handle_mouse_enter = Callback::from(move |_e: MouseEvent| {
        let element_option = cloned_node_ref.cast::<HtmlElement>();
        show_tooltip(
            cloned_ui_dispatch.clone(),
            element_option,
            cloned_text.clone(),
        );
    });

    let cloned_node_ref = element_ref.clone();
    let cloned_ui_dispatch = ui_dispatch.clone();
    let cloned_text = props.tooltip_text.clone();
    let handle_focus = Callback::from(move |_e: FocusEvent| {
        let element_option = cloned_node_ref.cast::<HtmlElement>();
        show_tooltip(
            cloned_ui_dispatch.clone(),
            element_option,
            cloned_text.clone(),
        );
    });

    let cloned_ui_dispatch = ui_dispatch.clone();
    let handle_mouse_leave = Callback::from(move |_| hide_tooltip(cloned_ui_dispatch.clone()));
    let cloned_ui_dispatch = ui_dispatch.clone();
    let handle_blur = Callback::from(move |_| hide_tooltip(cloned_ui_dispatch.clone()));

    html!(
    <div
        class="h-full w-full cursor-help"
        ref={element_ref}
        onmouseenter={handle_mouse_enter}
        onmouseleave={handle_mouse_leave}
        onfocus={handle_focus}
        onblur={handle_blur}
        tabindex={0}
        >
        {props.children.clone()}
    </div>
    )
}
