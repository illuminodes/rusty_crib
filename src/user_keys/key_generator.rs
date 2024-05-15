use crate::{
    js::web_utils::confirm_user_action,
    user_keys::key_context::{NostrKeyAction, NostrKeysContext},
};
use yew::prelude::*;

#[function_component(NostrKeyGen)]
pub fn nostr_key_gen() -> Html {
    let user_key_ctx = use_context::<NostrKeysContext>();
    if user_key_ctx.is_none() {
        return html! {};
    };
    let ctx_clone = user_key_ctx.clone().unwrap();
    let onclick = Callback::from(move |_| {
        ctx_clone.dispatch(NostrKeyAction::GenerateNewKey);
    });
    html! {
        <button {onclick}>{"Generate New"}</button>
    }
}

#[function_component(NostrKeyDelete)]
pub fn nostr_key_delete() -> Html {
    let user_key_ctx = use_context::<NostrKeysContext>();
    if user_key_ctx.is_none() {
        return html! {};
    };
    let ctx_clone = user_key_ctx.clone().unwrap();
    let onclick = Callback::from(move |_| {
        if let Ok(true) = confirm_user_action("Are you sure you want to delete this key?") {
            if let Ok(true) = confirm_user_action("WARNING! If you have not exported your key, you will lose access to your account. Are you sure you want to delete this key?") {
                ctx_clone.dispatch(NostrKeyAction::DeleteKey("privateKey".to_string()));
            }
        }
    });
    let delete_class = "bg-red";
    html! {
    <button {onclick} class={delete_class} >{"Delete Key"}</button>
    }
}
