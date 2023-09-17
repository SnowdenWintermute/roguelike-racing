use leptos::*;
use leptos_use::{use_websocket, UseWebSocketReadyState, UseWebsocketReturn};

#[component]
pub fn ws_test(cx: Scope) -> impl IntoView {
    let UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket(cx, "ws://127.0.0.1:8080/ws".to_string());

    let (displayed, set_displayed) = create_signal(cx, "".to_string());
    let (displayed_bytes, set_displayed_bytes) = create_signal(cx, Vec::new());

    create_effect(cx, move |_| {
        let gotten = message.get();
        log!("{:?}", gotten);
        let content = match gotten {
            Some(text) => text,
            None => "".to_string(),
        };
        log!("content: {}", content);
        set_displayed(content);
    });

    create_effect(cx, move |_| {
        let gotten = message_bytes.get();
        log!("{:?}", gotten);
        let content = match gotten {
            Some(bytes) => set_displayed_bytes(bytes),
            None => (),
        };
        log!("content: {:?}", content);
    });

    let send_message = move |_| {
        let m = "Hello, world!".to_string();
        send(m.clone());
    };

    let send_byte_message = move |_| {
        let m = b"Hello, world!\r\n".to_vec();
        send_bytes(m.clone());
    };

    let status = move || ready_state.get().to_string();

    let connected = move || ready_state.get() == UseWebSocketReadyState::Open;

    let open_connection = move |_| {
        open();
    };

    let close_connection = move |_| {
        close();
    };

    view! { cx,
        <div>
            <p>"status: " {status}</p>

            <button on:click=send_message disabled=move || !connected()>"Send"</button>
            <button on:click=send_byte_message disabled=move || !connected()>"Send bytes"</button>
            <button on:click=open_connection disabled=connected>"Open"</button>
            <button on:click=close_connection disabled=move || !connected()>"Close"</button>

            <p>"Receive message: " {move || format! {"{}", displayed.get()}}</p>
            <p>"Receive byte message: " {move || format! {"{:?}", displayed_bytes.get()}}</p>
        </div>
    }
}
