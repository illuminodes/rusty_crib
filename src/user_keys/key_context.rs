use gloo::console::error;
use nostro2::{
    nips::nip_46::{Nip46Commands, Nip46Request},
    notes::SignedNote,
    userkeys::UserKeys,
};
use std::rc::Rc;
use yew::{platform::spawn_local, prelude::*};

use crate::js::{
    indexed_db::{add_to_store, delete_from_store, retrieve_by_key, IdbDocument, IdbStores},
    web_utils::user_keys_to_js_value,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NostrKeys {
    user_keys: Option<UserKeys>,
}

impl NostrKeys {
    pub fn get_pubkey(&self) -> String {
        match self.user_keys.as_ref() {
            Some(keys) => keys.get_public_key(),
            None => String::new(),
        }
    }

    pub fn get_npub(&self) -> String {
        match self.user_keys.as_ref() {
            Some(keys) => keys.get_npub(),
            None => String::new(),
        }
    }

    pub fn has_keys(&self) -> bool {
        self.user_keys.is_some()
    }

    pub fn extract_nip46_command(&self, command: &SignedNote) -> Nip46Commands {
        let user_keys = self.user_keys.as_ref().unwrap();
        let newcommand = Nip46Request::get_request_command(command, &user_keys);
        newcommand
    }

    pub fn respond_to_nip46_command(&self, command: Nip46Commands) -> SignedNote {
        let user_keys = self.user_keys.as_ref().unwrap();
        let newcommand = Nip46Request::respond_to_command(&user_keys, command);
        newcommand
    }

    pub fn sign_note(&self, note: nostro2::notes::Note) -> SignedNote {
        let user_keys = self.user_keys.as_ref().unwrap();
        let newnote = user_keys.sign_nostr_event(note);
        newnote
    }
}

impl Reducible for NostrKeys {
    type Action = NostrKeyAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            NostrKeyAction::GenerateNewKey => {
                let new_secret_key = nostro2::userkeys::UserKeys::generate_extractable();
                let secret_bytes = new_secret_key.get_secret_key();
                spawn_local(async move {
                    let crypto_key = user_keys_to_js_value(&new_secret_key).await;
                    let _res = add_to_store(IdbStores::UserKeys(true), crypto_key).await;
                });
                let hex_key = secret_bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                let user_keys = UserKeys::new(&hex_key).unwrap();
                Rc::new(NostrKeys {
                    user_keys: Some(user_keys),
                })
            }
            NostrKeyAction::RetrieveKey(user_keys) => Rc::new(NostrKeys {
                user_keys: Some(user_keys),
            }),
            NostrKeyAction::UserImport(user_keys) => {
                let user_keys_extractable = UserKeys::new_extractable(&user_keys).unwrap();
                let user_keys = UserKeys::new(&user_keys).unwrap();
                spawn_local(async move {
                    let crypto_key = user_keys_to_js_value(&user_keys_extractable).await;
                    let _ = add_to_store(IdbStores::UserKeys(true), crypto_key).await;
                });

                Rc::new(NostrKeys {
                    user_keys: Some(user_keys),
                })
            }
            NostrKeyAction::DeleteKey(_key) => {
                spawn_local(async move {
                    let _res = delete_from_store(IdbStores::UserKeys(true), "privateKey").await;
                });
                Rc::new(NostrKeys { user_keys: None })
            }
        }
    }
}

pub type NostrKeysContext = UseReducerHandle<NostrKeys>;

pub enum NostrKeyAction {
    GenerateNewKey,
    RetrieveKey(UserKeys),
    UserImport(String),
    DeleteKey(String),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NostrKeyChildren {
    pub children: Children,
}

#[function_component(KeyContext)]
pub fn key_handler(props: &NostrKeyChildren) -> Html {
    let ctx = use_reducer(|| NostrKeys { user_keys: None });

    let ctx_clone = ctx.clone();
    use_effect_with((), |_| {
        spawn_local(async move {
            let user_key = retrieve_by_key(IdbStores::UserKeys(false), "privateKey").await;
            match user_key {
                Ok(IdbDocument::UserKey(user_keys)) => {
                    ctx_clone.dispatch(NostrKeyAction::RetrieveKey(user_keys));
                }
                Err(e) => {
                    error!("No user keys found", e);
                }
                _ => {}
            }
        });
        || {}
    });

    html! {
        <ContextProvider<NostrKeysContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<NostrKeysContext>>
    }
}
