use gloo::{
    console::{error, log},
    utils::format::JsValueSerdeExt,
};
use js_sys::Object;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use yew::{platform::spawn_local, prelude::*};

use crate::js::indexed_db::{add_to_store, delete_from_store, retrieve_all_from_store, IdbStores};

pub fn js_array_to_relay_list(array_buffer: JsValue) -> Result<NostrRelayList, String> {
    let cast_vec = array_buffer.dyn_into::<js_sys::Array>().unwrap();
    let mut user_relays = vec![];
    for i in 0..cast_vec.length() {
        let relay = cast_vec.get(i);
        let relay = relay.dyn_into::<Object>().unwrap();
        let user_relay: UserRelay = relay.into_serde().unwrap();
        user_relays.push(user_relay);
    }
    Ok(NostrRelayList { user_relays })
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserRelay {
    pub url: String,
    pub read: bool,
    pub write: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NostrRelayList {
    user_relays: Vec<UserRelay>,
}

impl NostrRelayList {
    pub fn get_relays(&self) -> Vec<UserRelay> {
        self.user_relays.clone()
    }
}

impl Reducible for NostrRelayList {
    type Action = RelayListAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            RelayListAction::InitRelays(relays) => {
                let mut new_relays = self.user_relays.clone();
                new_relays.extend(relays);
                Rc::new(NostrRelayList {
                    user_relays: new_relays,
                })
            }
            RelayListAction::AddRelay(relay) => {
                let mut new_relays = self.user_relays.clone();
                new_relays.insert(0, relay.clone());
                spawn_local(async move {
                    let js_relay = JsValue::from_serde(&relay).unwrap();
                    log!("Adding relay to store", &js_relay);
                    if let Err(e) = add_to_store(IdbStores::UserRelays, js_relay).await {
                        error!(e)
                    }
                });
                Rc::new(NostrRelayList {
                    user_relays: new_relays,
                })
            }
            RelayListAction::RemoveRelay(relay) => {
                let mut new_relays = self.user_relays.clone();
                new_relays.retain(|r| r != &relay);
                spawn_local(async move {
                    let _relay_list = delete_from_store(IdbStores::UserRelays, &relay.url).await;
                });
                Rc::new(NostrRelayList {
                    user_relays: new_relays,
                })
            }
            RelayListAction::ModifyRelay(relay) => {
                let mut new_relays = self.user_relays.clone();
                new_relays.iter_mut().find(|r| r.url == relay.url).map(|r| {
                    r.read = relay.read;
                    r.write = relay.write;
                });
                spawn_local(async move {
                    let js_relay = JsValue::from_serde(&relay).unwrap();
                    if let Err(e) = add_to_store(IdbStores::UserRelays, js_relay).await {
                        error!(e)
                    }
                });
                Rc::new(NostrRelayList {
                    user_relays: new_relays,
                })
            }
        }
    }
}

pub type NostrRelaysContext = UseReducerHandle<NostrRelayList>;

pub enum RelayListAction {
    InitRelays(Vec<UserRelay>),
    AddRelay(UserRelay),
    RemoveRelay(UserRelay),
    ModifyRelay(UserRelay),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RelayListChildren {
    pub children: Children,
}

#[function_component(RelayContext)]
pub fn relay_context(props: &RelayListChildren) -> Html {
    let ctx = use_reducer(|| NostrRelayList {
        user_relays: vec![],
    });

    let ctx_clone = ctx.clone();
    use_effect_with((), move |_| {
        let ctx = ctx_clone.clone();
        spawn_local(async move {
            let relay_list = retrieve_all_from_store(IdbStores::UserRelays).await;
            if relay_list.is_err() {
                return;
            }
            let relay_list = relay_list.unwrap();
            let cast_list = js_array_to_relay_list(relay_list);
            let cast_list = cast_list.unwrap();

            ctx.dispatch(RelayListAction::InitRelays(cast_list.user_relays));
        });
        || {}
    });

    html! {
        <ContextProvider<NostrRelaysContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<NostrRelaysContext>>
    }
}
