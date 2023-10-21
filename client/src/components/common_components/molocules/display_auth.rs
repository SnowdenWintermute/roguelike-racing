use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::auth_store::AuthStore;

#[function_component(DisplayAuth)]
pub fn display_auth() -> Html {
    let (auth_state, _) = use_store::<AuthStore>();
    let username = format!(
        "Username: {}",
        auth_state.username.as_deref().unwrap_or_default()
    );
    let password = format!(
        "Password: {}",
        auth_state.password.as_deref().unwrap_or_default()
    );
    let is_authenticated = format!("Authenticated: {}", auth_state.is_authenticated);

    html!(
        <div>
            <h2>{"Auth data"}</h2>
            <div>{username}</div>
            <div>{password}</div>
            <div>{is_authenticated}</div>
        </div>
    )
}
