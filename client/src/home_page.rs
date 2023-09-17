// use futures_util::SinkExt;
use leptos::*;
// use pharos::{Observable, ObserveConfig};
// use ws_stream_wasm::*;

// async fn connect_ws() -> Result<(WsMeta, WsStream), WsErr> {
//     log!("attempting connection");
//     let (mut wsmeta, mut wsio) = match WsMeta::connect("ws://127.0.0.1:8080/ws", None).await {
//         Ok(conn) => conn,
//         Err(e) => {
//             error!("{}", e);
//             return Err(e);
//         }
//     };
//     log!("connection made");

//     wsio.send(WsMessage::Text("some message text".to_owned()))
//         .await
//         .expect("couldn't send");
//     Ok((wsmeta, wsio))
// }

/// Renders the home page of your application.
#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    // let connection = create_local_resource(cx, || (), |_| async move { connect_ws().await });
    // let is_loading = connection.loading();

    // let (mut wsmeta, mut wsio) = match WsMeta::connect("ws://127.0.0.1:8080/ws", None).await {
    //     Ok(conn) => conn,
    //     Err(e) => {
    //         error!("{}", e);
    //         return Err(e);
    //     }
    // };

    view! { cx,
        <h1>"Leptos app"</h1>
    }
}

// let current_connection = move || {
//     connection.with(cx, |conn| match conn {
//         Ok((wsMeta, wsStream)) => {
//             return Ok((wsMeta, wsStream));
//         }
//         Err(err) => {
//             return Err(err);
//         }
//     });
// };
//
//

