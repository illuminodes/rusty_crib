use crate::{
    atoms::{buttons::TextInput, toasts::error_handler},
    js::web_utils::event_to_html_input,
    user_keys::key_context::{NostrKeyAction, NostrKeysContext},
};
use nostro2::userkeys::UserKeys;
use yew::prelude::*;

#[function_component(StringKeyImport)]
pub fn new_user_key() -> Html {
    let user_keys_context = use_context::<NostrKeysContext>();
    if user_keys_context.is_none() {
        return html! {};
    }
    let user_keys_context = user_keys_context.unwrap();

    let imported_key_state = use_state(|| String::new());
    let imported_key_clone = imported_key_state.clone();
    let imported_key_onchange = Callback::from(move |e: Event| {
        let value = event_to_html_input(e).value();
        imported_key_clone.set(value);
    });

    let error_notice = use_state(|| html! {});
    let error_clone = error_notice.clone();
    let onclick = Callback::from(move |_| {
        let key = imported_key_state.to_owned().to_string();
        let user_keys = UserKeys::new(&key);
        if user_keys.is_err() {
            error_handler(error_clone.clone(), "Invalid Key!".to_string());
            return;
        }
        user_keys_context.dispatch(NostrKeyAction::UserImport(key));
    });

    html! {
    <>
        <div class="flex flex-row items-center gap-4">
            <TextInput placeholder="Hex or nsec" onchange={imported_key_onchange} input_type="text" extra_class="" />
            <button {onclick}>{"Save"}</button>
        </div>
        {(*error_notice).clone()}
    </>
    }
}

#[function_component(MnemonicKeyImport)]
pub fn mnemonic_key_import() -> Html {
    let user_keys_context = use_context::<NostrKeysContext>();
    if user_keys_context.is_none() {
        return html! {};
    }
    let user_keys_context = user_keys_context.unwrap();

    let imported_key_state = use_state(|| String::new());
    let imported_key_clone = imported_key_state.clone();
    let onchange = Callback::from(move |e: Event| {
        let value = event_to_html_input(e).value();
        imported_key_clone.set(value);
    });

    let error_notice = use_state(|| html! {});
    let error_clone = error_notice.clone();
    let imported_key_state_clone = imported_key_state.clone();
    let onclick = Callback::from(move |_| {
        let key = (*imported_key_state_clone).to_string();
        let user_keys = UserKeys::parse_mnemonic(&key, true);
        if user_keys.is_err() {
            error_handler(error_clone.clone(), "Invalid Key!".to_string());
            return;
        }
        let key = user_keys
            .unwrap()
            .get_secret_key()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        user_keys_context.dispatch(NostrKeyAction::UserImport(key));
    });

    html! {
    <>
        <div class="flex flex-row items-center gap-4">
            <TextInput placeholder="24 word mnemonic" {onchange} input_type="text" extra_class="" />
            <button {onclick} >{"Save"}</button>
        </div>
        {(*error_notice).clone()}
    </>
    }
}
