use gloo::console::log;
use js_sys::{ArrayBuffer, Object, Uint8Array};
use nostro2::userkeys::UserKeys;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{AesKeyGenParams, Event, SubtleCrypto, ServiceWorkerContainer};

pub fn confirm_user_action(message: &str) -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    window.confirm_with_message(message)
}

fn crypto_subtle() -> SubtleCrypto {
    let window = web_sys::window().unwrap();
    window.crypto().unwrap().subtle()
}

fn user_keys_to_object(user_keys: &UserKeys) -> Object {
    let secret_bytes = user_keys.get_secret_key();
    let array = Uint8Array::from(&secret_bytes[..]);
    array.buffer().into()
}

pub async fn user_keys_to_js_value(user_keys: &UserKeys) -> JsValue {
    let key_object = user_keys_to_object(user_keys);
    let crypto = crypto_subtle();
    let algo = AesKeyGenParams::new("AES-GCM", 256);
    let usages = js_array(&["encrypt", "decrypt"]);
    let key = crypto
        .import_key_with_object("raw", &key_object, &algo, true, &usages)
        .unwrap();
    let key: JsValue = JsFuture::from(key).await.unwrap();
    key
}

pub async fn js_value_to_user_keys(
    js_value: JsValue,
    extractable: bool,
) -> Result<UserKeys, JsValue> {
    let crypto = crypto_subtle();
    let key = JsFuture::from(crypto.export_key("raw", &js_value.into())?).await?;
    let key_array: ArrayBuffer = key.into();
    let key_array = Uint8Array::new(&key_array);
    let key_array = key_array.to_vec();
    let key_hex = key_array
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    match extractable {
        true => Ok(UserKeys::new_extractable(&key_hex).unwrap()),
        false => Ok(UserKeys::new(&key_hex).unwrap()),
    }
}

pub fn js_array(values: &[&str]) -> JsValue {
    return JsValue::from(
        values
            .iter()
            .map(|x| JsValue::from_str(x))
            .collect::<js_sys::Array>(),
    );
}

pub fn event_to_html_input(event: Event) -> web_sys::HtmlInputElement {
    event
        .target()
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
}

fn sw() -> web_sys::ServiceWorkerContainer {
    web_sys::window().unwrap().navigator().service_worker()
}

async fn install_service_worker(sw: &ServiceWorkerContainer) {
    let register = sw.register("serviceWorker.js");
    let register = JsFuture::from(register).await;
    match register {
        Ok(res) => {
            log!("Service worker registered:", res);
        },
        Err(e) => {
            log!("Service worker registration failed:", e);
        },
    }
}

pub async fn init_service_worker() {
    let sw = sw();
    install_service_worker(&sw).await;
}
