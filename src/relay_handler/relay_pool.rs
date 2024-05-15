use crate::{
    relay_handler::{relay_component::RelayComponent, relay_notifier::Nip46Notifier},
    user_keys::key_context::NostrKeysContext,
    user_relays::relay_context::NostrRelaysContext,
};
use yew::prelude::*;

#[function_component(RelayPool)]
pub fn relay_pool() -> Html {
    let relay_list_ctx = use_context::<NostrRelaysContext>();
    let user_keys = use_context::<NostrKeysContext>();

    if relay_list_ctx.is_none() || user_keys.is_none() {
        return html! {};
    };

    let pool_state = use_state(|| html! {});
    let user_pk = user_keys.clone().unwrap().get_pubkey();
    use_effect_with((relay_list_ctx.clone(), pool_state.clone()), move |deps| {
        let relays = deps.0.as_ref().unwrap().get_relays();
        let pool_clone = deps.1.clone();
        let pool_html = relays.iter().map(|relay| {
            html! {
                <RelayComponent user_relay={relay.clone()} user_pk={user_pk.clone()}>
                    <Nip46Notifier />
                </RelayComponent>
            }
        });
        pool_clone.set(pool_html.collect());
        move || {
            pool_clone.set(html! {});
        }
    });

    let relay_list = relay_list_ctx.clone().unwrap().get_relays();
    let relay_msg = match relay_list.len() {
        0 => "No relays found".to_string(),
        1 => "Waiting for requests from 1 relay...".to_string(),
        _ => format!("Waiting for requests from {} relays...", relay_list.len()),
    };

    html! {
        <div class="flex flex-col flex-1">
            {if relay_list.is_empty() {
                html! {
                <>
                    <p>{"No relays found."}</p>
                    <p>{"Please add a relay."}</p>
                </>
                }
            } else {
                html! {
                <div class="flex flex-col flex-1 gap-8">
                    <p>{relay_msg}</p>
                    <ul class="flex flex-col flex-1 gap-8 items-center">
                        {(*pool_state).clone()}
                    </ul>
                </div>
                }
            }}
        </div>
    }
}
