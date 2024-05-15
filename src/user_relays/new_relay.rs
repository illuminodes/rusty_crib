use crate::{
    atoms::buttons::TextInput,
    user_relays::relay_context::{NostrRelaysContext, UserRelay},
};
use nostro2::relays::NostrRelay;
use yew::{platform::spawn_local, prelude::*};

use super::relay_context::RelayListAction;
use crate::{atoms::toasts::error_handler, js::web_utils::event_to_html_input};

#[function_component(NewRelay)]
pub fn new_relay() -> Html {
    let relay_list_context = use_context::<NostrRelaysContext>();
    if relay_list_context.is_none() {
        return html! {};
    }

    let relay_url_state = use_state(|| String::new());
    let url_clone = relay_url_state.clone();
    let relay_onchange = Callback::from(move |e: Event| {
        let value = event_to_html_input(e).value();
        url_clone.set(value);
    });

    let url_clone = relay_url_state.clone();
    let relay_list_context = relay_list_context.clone().unwrap();

    let error_notice = use_state(|| html! {});
    let error_clone = error_notice.clone();
    let onclick = Callback::from(move |_| {
        if url_clone.is_empty() {
            error_handler(error_clone.clone(), "URL cannot be empty!".to_string());
            return;
        }
        let mut url = format!("{}", (*url_clone).to_lowercase());
        // Check that the wss:// is not repeated
        if !url.starts_with("wss://") {
            url = format!("wss://{}", url);
        }
        let relay_list_context = relay_list_context.clone();
        let error_clone = error_clone.clone();
        spawn_local(async move {
            let relay_check = NostrRelay::new(&url).await;
            if relay_check.is_err() {
                error_handler(error_clone, "Invalid relay address!".to_string());
                return;
            }
            let _ = relay_check.unwrap().close().await;
            let relay = UserRelay {
                url,
                read: true,
                write: true,
            };
            relay_list_context.dispatch(RelayListAction::AddRelay(relay));
        });
    });
    let svg_class = "h-12 w-12 p-2 text-white border-2 border-white rounded bg-nostr-dark";

    html! {
    <>
        <div class="flex flex-row gap-4 sm:gap-8 font-semibold items-center">
            <TextInput placeholder="Add relay address" onchange={relay_onchange} input_type="text" extra_class="" />
            <svg {onclick} class={svg_class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-7 7V5"/>
            </svg>

        </div>
        {(*error_notice).clone()}
    </>
    }
}
