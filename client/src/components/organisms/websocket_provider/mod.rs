use common::errors::AppError;
use gloo::console::log;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[derive(Default, Clone)]
pub struct CustomFormData {}

#[function_component(WebsocketProvider)]
pub fn websocket_provider(props: &Props) -> Html {
    let websocket_state: UseStateHandle<Option<WebSocket>> = use_state(|| None);

    use_effect_with((), move |_| {
        let websocket = WebSocket::new("ws://127.0.0.1:8081/ws");
        match websocket {
            Ok(websocket_success) => {
                // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
                websocket_success.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = websocket_success.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    let result = (|| -> Result<(), AppError> {
                        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                            let array = js_sys::Uint8Array::new(&abuf);
                            let byte_slice = &array.to_vec()[..];
                        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                            log!("message event, received Text: {:?}", txt);
                        } else {
                            log!("message event, received Unknown: {:?}", e.data());
                        }
                        Ok(())
                    })();
                    match result {
                        Err(app_error) => {
                            log!("unhandled error");
                            // alerts::set_alert(
                            //     alerts,
                            //     app_error.message.clone(),
                            //     &mut last_alert_id,
                            // );
                        }
                        Ok(()) => (),
                    };
                });
                cloned_ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget(); // forget the callback to keep it alive
                websocket_state.set(Some(websocket_success));
            }
            _ => println!("websocket failed to create"),
        }
    });

    html!(
        <div>
            {props.children.clone()}
        </div>
    )
}
