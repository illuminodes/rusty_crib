use gloo::{
    console::error,
    utils::format::JsValueSerdeExt,
};
use nostro2::{notes::SignedNote, userkeys::UserKeys};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{IdbObjectStore, IdbObjectStoreParameters, IdbTransactionMode};
use yew::platform::{
    pinned::oneshot::{self, Receiver},
    spawn_local,
};

use crate::user_relays::relay_context::UserRelay;

use super::web_utils::js_value_to_user_keys;

const DB_NAME: &str = "BunkerDB";
const DB_VERSION: u32 = 1;

#[derive(Clone, Debug)]
pub enum IdbStores {
    UserKeys(bool),
    UserRelays,
    UserNotes,
}

impl ToString for IdbStores {
    fn to_string(&self) -> String {
        match self {
            IdbStores::UserKeys(_) => "userKeys".to_string(),
            IdbStores::UserRelays => "userRelays".to_string(),
            IdbStores::UserNotes => "userNotes".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum IdbDocument {
    UserKey(UserKeys),
    UserRelay(UserRelay),
    UserNote(SignedNote),
}

pub async fn add_to_store(store_name: IdbStores, value: JsValue) -> Result<(), JsValue> {
    let object_store_request = request_store_open(store_name.clone())?;
    let object_store = object_store_request
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    match store_name {
        IdbStores::UserKeys(_) => {
            if let Err(request) =
                object_store.put_with_key(&value, &JsValue::from_str("privateKey"))
            {
                error!("Error adding to store:", request);
            }
        }
        IdbStores::UserRelays => {
            if let Err(request) = object_store.put(&value) {
                error!("Error adding to store:", request);
            }
        }
        IdbStores::UserNotes => {
            if let Err(request) = object_store.put(&value) {
                error!("Error adding to store:", request);
            }
        }
    }
    Ok(())
}

pub async fn delete_from_store(store_name: IdbStores, key: &str) -> Result<(), JsValue> {
    let object_store_request = request_store_open(store_name.clone())?;
    let object_store = object_store_request
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let request = object_store.delete(&JsValue::from_str(key))?;
    let req_clone = request.clone();
    let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
        let _result: JsValue = req_clone.result().unwrap();
    });
    request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
    Ok(())
}

pub async fn retrieve_by_key(store_name: IdbStores, key: &str) -> Result<IdbDocument, JsValue> {
    let object_store_request = request_store_open(store_name.clone())?;
    let (sender, receiver) = oneshot::channel();
    let object_store = object_store_request
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let request = object_store.get(&JsValue::from_str(key))?;
    let req_clone = request.clone();
    match store_name {
        IdbStores::UserKeys(extractable) => {
            let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
                let result: JsValue = req_clone.result().unwrap();
                if result.is_null() || result.is_undefined() {
                    return;
                }
                spawn_local(async move {
                    let user_keys = js_value_to_user_keys(result, extractable).await.unwrap();
                    let idb_doc = IdbDocument::UserKey(user_keys);
                    let _ = sender.send(idb_doc);
                });
            });
            request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
        }
        IdbStores::UserRelays => {
            let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
                let result: JsValue = req_clone.result().unwrap();
                let user_relay: UserRelay = result.into_serde().unwrap();
                let idb_doc = IdbDocument::UserRelay(user_relay);
                let _ = sender.send(idb_doc);
            });
            request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
        }
        IdbStores::UserNotes => {
            let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
                let result: JsValue = req_clone.result().unwrap();
                let signed_note: SignedNote = result.into_serde().unwrap();
                let idb_doc = IdbDocument::UserNote(signed_note);
                let _ = sender.send(idb_doc);
            });
            request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
        }
    }
    let idb_doc = receiver
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(idb_doc)
}

pub async fn retrieve_all_from_store(store_name: IdbStores) -> Result<JsValue, JsValue> {
    let (sender, receiver) = oneshot::channel();
    let object_store_request = request_store_open(store_name.clone())?
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let request = object_store_request.get_all()?;
    let req_clone = request.clone();
    match store_name {
        IdbStores::UserKeys(extractable) => match extractable {
            true => {
                let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
                    let result: JsValue = req_clone.result().unwrap();
                    let _ = sender.send(result);
                });
                request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
            }
            false => {
                let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
                    let result: JsValue = req_clone.result().unwrap();
                    let _ = sender.send(result);
                });
                request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
            }
        },
        IdbStores::UserRelays => {
            let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
                let result: JsValue = req_clone.result().unwrap();
                let _ = sender.send(result);
            });
            request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
        }
        IdbStores::UserNotes => {
            let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
                let result: JsValue = req_clone.result().unwrap();
                let _ = sender.send(result);
            });
            request.set_onsuccess(Some(on_success.dyn_ref().unwrap()));
        }
    }

    let result = receiver.await.unwrap();
    Ok(result)
}

fn upgrade_db(event: web_sys::Event) -> Result<(), JsValue> {
    if event.target().is_none() {
        return Err(JsValue::from_str("Error upgrading database"));
    };
    let target = event.target().unwrap();
    let db = target
        .dyn_into::<web_sys::IdbOpenDbRequest>()?
        .result()?
        .dyn_into::<web_sys::IdbDatabase>()?;
    db.create_object_store("userKeys")?;
    let mut user_relay_params = IdbObjectStoreParameters::new();
    user_relay_params.key_path(Some(&JsValue::from_str("url")));
    db.create_object_store_with_optional_parameters("userRelays", &user_relay_params)?;
    let mut user_note_params = IdbObjectStoreParameters::new();
    user_note_params.key_path(Some(&JsValue::from_str("id")));
    db.create_object_store_with_optional_parameters("userNotes", &user_note_params)?;
    Ok(())
}

fn request_db_open() -> Result<web_sys::IdbOpenDbRequest, JsValue> {
    let window = web_sys::window().unwrap();
    if let Some(idb_factory) = window.indexed_db()? {
        let idb_open_request = idb_factory.open_with_u32(DB_NAME, DB_VERSION)?;
        let on_upgrade_needed = Closure::once_into_js(move |event: web_sys::Event| {
            if let Err(e) = upgrade_db(event) {
                error!(&e);
            }
        });
        idb_open_request.set_onupgradeneeded(Some(on_upgrade_needed.as_ref().unchecked_ref()));
        Ok(idb_open_request)
    } else {
        Err(JsValue::from_str("IndexedDB not supported"))
    }
}

fn request_store_open(store_name: IdbStores) -> Result<Receiver<IdbObjectStore>, JsValue> {
    let idb_open_request = request_db_open()?;
    let store_name_str = store_name.to_string();
    let idb_clone = idb_open_request.clone();
    let (sender, receiver) = oneshot::channel();
    let on_success = Closure::once_into_js(move |_event: web_sys::Event| {
        let db: web_sys::IdbDatabase = idb_clone.result().unwrap().dyn_into().unwrap();
        let transaction = db
            .transaction_with_str_and_mode(&store_name_str, IdbTransactionMode::Readwrite)
            .unwrap();
        let object_store = transaction.object_store(&store_name_str).unwrap();
        let _ = sender.send(object_store);
    });
    idb_open_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    Ok(receiver)
}
