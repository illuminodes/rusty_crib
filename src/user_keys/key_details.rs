use yew::prelude::*;
use crate::user_keys::key_context::NostrKeysContext;

#[function_component(NostrKeyDetails)]
pub fn nostr_key_details() -> Html {
    let user_keys = use_context::<NostrKeysContext>();
    if user_keys.is_none() {
        return html! {};
    };
    let user_keys = user_keys.unwrap();
    if user_keys.get_pubkey().is_empty() {
        return html! {
        <div class="flex flex-col font-bold">
            <p>{"No Key Found"}</p>
        </div>
        };
    };
    let pubkey = &user_keys.get_npub()[..16];
    html! {
    <div class="flex flex-col font-bold">
        <p>{format!("Signing as {}...", pubkey)}</p>
    </div>
    }
}
