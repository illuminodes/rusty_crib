use crate::{
    atoms::buttons::{AnimatedButton, CopyOption},
    js::bindings::clipboardCopy,
    js::indexed_db::{retrieve_by_key, IdbDocument, IdbStores},
    user_keys::key_context::NostrKeysContext,
};
use gloo_timers::callback::Timeout;
use yew::{platform::spawn_local, prelude::*};

#[function_component(NostrPrivateKeyExport)]
pub fn nostr_key_export() -> Html {
    let user_key_ctx = use_context::<NostrKeysContext>();
    if user_key_ctx.is_none() {
        return html! {};
    };

    let clicked_state_hex = use_state(|| false);
    let clicked_state_clone = clicked_state_hex.clone();
    let onclick_hex = Callback::from(move |_| {
        let clicked_state_clone = clicked_state_clone.clone();
        clicked_state_clone.set(true);
        spawn_local(async move {
            let user_keys = retrieve_by_key(IdbStores::UserKeys(true), "privateKey").await;
            if let Ok(IdbDocument::UserKey(user_keys)) = user_keys {
                let key_hex = user_keys
                    .get_secret_key()
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();

                let _ = clipboardCopy(&key_hex).await;
                let _ = Timeout::new(210, move || {
                    clicked_state_clone.set(false);
                })
                .forget();
            }
        });
    });

    let clicked_state_nsec = use_state(|| false);
    let clicked_nsec_clone = clicked_state_nsec.clone();
    let onclick_nsec = Callback::from(move |_| {
        let clicked_state_clone = clicked_nsec_clone.clone();
        clicked_state_clone.set(true);
        spawn_local(async move {
            let user_keys = retrieve_by_key(IdbStores::UserKeys(true), "privateKey").await;
            if let Ok(IdbDocument::UserKey(user_keys)) = user_keys {
                let key_nsec = user_keys.get_nsec();
                let _ = clipboardCopy(&key_nsec).await;
                let _ = Timeout::new(210, move || {
                    clicked_state_clone.set(false);
                })
                .forget();
            }
        });
    });

    html! {
        <div class={"flex flex-row gap-4 flex-wrap sm:gap-8 w-full pl-8 pr-16 justify-between"}>
            <CopyOption text="Hex" onclick={onclick_hex} class={""} />
            <CopyOption text="Nsec" onclick={onclick_nsec} class={""} />
        </div>
    }
}

#[function_component(NostrMnemonicKeyExport)]
pub fn nostr_key_export() -> Html {
    let user_key_ctx = use_context::<NostrKeysContext>();
    if user_key_ctx.is_none() {
        return html! {};
    };

    let mnemonic_text = use_state(|| String::new());

    let mnemonic_text_clone = mnemonic_text.clone();
    let onclick_mnemonic = Callback::from(move |_| {
        let mnemonic_text_clone = mnemonic_text_clone.clone();
        spawn_local(async move {
            let user_keys = retrieve_by_key(IdbStores::UserKeys(true), "privateKey").await;
            if let Ok(IdbDocument::UserKey(user_keys)) = user_keys {
                let mnemonic = user_keys.get_mnemonic_phrase();
                let _ = clipboardCopy(&mnemonic).await;
                mnemonic_text_clone.set(mnemonic);
            }
        });
    });

    let mnemonic_text_clone = mnemonic_text.clone();
    let onclick_mnemonic_spanish = Callback::from(move |_| {
        let mnemonic_text_clone = mnemonic_text_clone.clone();
        spawn_local(async move {
            let user_keys = retrieve_by_key(IdbStores::UserKeys(true), "privateKey").await;
            if let Ok(IdbDocument::UserKey(user_keys)) = user_keys {
                let mnemonic = user_keys.get_mnemonic_spanish();
                let _ = clipboardCopy(&mnemonic).await;
                mnemonic_text_clone.set(mnemonic);
            }
        });
    });

    html! {
        <div class="flex flex-col flex-1 gap-8 sm:gap-16">
            <div class={"flex flex-row gap-4 flex-wrap sm:gap-8 w-full pl-8 pr-16 justify-between"}>
                <AnimatedButton button_text={"Mnemonic".to_string()} onclick_text={"Copied!".to_string()} onclick={onclick_mnemonic} />
                <AnimatedButton button_text={"Spanish Mnemonic".to_string()} onclick_text={"Copied!".to_string()} onclick={onclick_mnemonic_spanish} />
            </div>
            {if !mnemonic_text.is_empty() {
                html! {
                    <div class="flex flex-col gap-4 font-bold text-wrap flex-wrap">
                        <p class="max-w-72">{&*mnemonic_text}</p>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

#[function_component(NostrPublicKeyExport)]
pub fn nostr_public_key_export() -> Html {
    let user_key_ctx = use_context::<NostrKeysContext>();
    if user_key_ctx.is_none() {
        return html! {};
    };

    let onclick_npub = Callback::from(move |_| {
        spawn_local(async move {
            let user_keys = retrieve_by_key(IdbStores::UserKeys(false), "privateKey").await;
            if let Ok(IdbDocument::UserKey(user_keys)) = user_keys {
                let key_array = user_keys.get_npub();
                let _ = clipboardCopy(&key_array).await;
            }
        });
    });

    let onclick_hex = Callback::from(move |_| {
        spawn_local(async move {
            let user_keys = retrieve_by_key(IdbStores::UserKeys(false), "privateKey").await;
            if let Ok(IdbDocument::UserKey(user_keys)) = user_keys {
                let key_hex = user_keys.get_public_key();
                let _ = clipboardCopy(&key_hex).await;
            }
        });
    });

    html! {
        <div class="flex flex-row gap-16 sm:gap-24 pl-8 pr-16">
            <CopyOption text="Hex" onclick={onclick_hex} class={""} />
            <CopyOption text="Npub" onclick={onclick_npub} class={""} />
        </div>
    }
}
