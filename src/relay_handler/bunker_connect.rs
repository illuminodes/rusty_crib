use yew::{platform::spawn_local, prelude::*};

use crate::{
    atoms::buttons::CopyOption,
    js::bindings::clipboardCopy,
    user_keys::key_context::NostrKeysContext,
    user_relays::relay_context::NostrRelaysContext,
};

#[function_component(BunkerConnectExport)]
pub fn bunker_connect() -> Html {
    let user_key_ctx = use_context::<NostrKeysContext>();
    let user_relays = use_context::<NostrRelaysContext>();

    if user_key_ctx.is_none() || user_relays.is_none() {
        return html! {};
    };
    let key_ctx = user_key_ctx.unwrap();
    let relays_ctx = user_relays.unwrap();
    if !key_ctx.has_keys() || relays_ctx.get_relays().is_empty() {
        return html! {};
    }
    let onclick = Callback::from(move |_| {
        let user_pk = key_ctx.get_pubkey();
        let relay_list = relays_ctx.get_relays();
        let mut bunker_string = format!("bunker://{}?", user_pk);
        for relay in relay_list {
            bunker_string.push_str(&format!("relay={}&", relay.url));
        }
        spawn_local(async move {
            let _ = clipboardCopy(&bunker_string).await;
        });
    });
    let text = "Bunker URL";

    html! {
        <CopyOption {text} onclick={onclick} class={""} />
    }
}
