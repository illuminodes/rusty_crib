use crate::{
    js::web_utils::confirm_user_action,
    user_relays::relay_context::{NostrRelaysContext, UserRelay}, atoms::buttons::DeleteSvg,
};
use yew::prelude::*;

#[function_component(RelayList)]
pub fn relay_list() -> Html {
    let relay_list_ctx = use_context::<NostrRelaysContext>();

    let relay_list: UseStateHandle<Vec<UserRelay>> = use_state(|| vec![]);

    let relay_list_clone = relay_list.clone();
    use_effect_with(relay_list_ctx.clone(), move |deps| {
        let relays = deps.as_ref().unwrap().get_relays();
        relay_list_clone.set(relays);
        move || {
            relay_list_clone.set(vec![]);
        }
    });

    if relay_list.is_empty() {
        return html! {
        <div>
            <p>{"No relays found."}</p>
            <p>{"Please add a relay."}</p>
        </div>
        };
    }
    html! {
        <ul class="flex flex-col gap-4 sm:gap-8">
        {for relay_list.iter().map(|relay| {
            html! {
            <li>
                <div class="flex flex-row justify-between w-full sm:gap-8 items-center font-bold text-xs sm:text-sm md:text-base">
                    <p class="">{format!("{}", &relay.url[6..])}</p>
                    <div class={"justify-self-end"}>
                        <RelayOptions relay={relay.clone()} />
                    </div>
                </div>
            </li>
            }
        })}
        </ul>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RelayOptionsProps {
    pub relay: UserRelay,
}

use crate::user_relays::relay_context::RelayListAction;
#[function_component(RelayOptions)]
pub fn relay_options(props: &RelayOptionsProps) -> Html {
    let relay_list_ctx = use_context::<NostrRelaysContext>();
    let relay_list_clone = relay_list_ctx.clone().unwrap();
    let relay_to_delete = props.relay.clone();
    let onclick_delete = Callback::from(move |_| {
        if let Ok(true) = confirm_user_action("Are you sure you want to delete this relay?") {
            relay_list_clone.dispatch(RelayListAction::RemoveRelay(relay_to_delete.clone()));
        }
    });

    let relay_read_change = props.relay.clone();
    let relay_list_clone = relay_list_ctx.clone().unwrap();
    let onclick_change_read = Callback::from(move |_| {
        let mut new_relay = relay_read_change.clone();
        new_relay.read = !new_relay.read;
        relay_list_clone.dispatch(RelayListAction::ModifyRelay(new_relay));
    });

    let relay_write_change = props.relay.clone();
    let relay_list_clone = relay_list_ctx.clone().unwrap();
    let onclick_change_write = Callback::from(move |_| {
        let mut new_relay = relay_write_change.clone();
        new_relay.write = !new_relay.write;
        relay_list_clone.dispatch(RelayListAction::ModifyRelay(new_relay));
    });

    let true_prop_class: &str = "bg-green border-4 border-white rounded-full";
    let false_prop_class: &str = "bg-red border-4 border-white rounded-full";
    let relay_read_class = if props.relay.read {
        true_prop_class
    } else {
        false_prop_class
    };
    let relay_write_class = if props.relay.write {
        true_prop_class
    } else {
        false_prop_class
    };

    html! {
    <div class="flex gap-4 flex-row justify-end">

        <svg onclick={onclick_change_read} class={relay_read_class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24">
            <path fill-rule="evenodd" d="M13 11.15V4a1 1 0 1 0-2 0v7.15L8.78 8.374a1 1 0 1 0-1.56 1.25l4 5a1 1 0 0 0 1.56 0l4-5a1 1 0 1 0-1.56-1.25L13 11.15Z" clip-rule="evenodd"/>
            <path fill-rule="evenodd" d="M9.657 15.874 7.358 13H5a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2.358l-2.3 2.874a3 3 0 0 1-4.685 0ZM17 16a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17Z" clip-rule="evenodd"/>
        </svg>

        <svg onclick={onclick_change_write} class={relay_write_class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24">
            <path d="M2.038 5.61A2.01 2.01 0 0 0 2 6v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V6c0-.12-.01-.238-.03-.352l-.866.65-7.89 6.032a2 2 0 0 1-2.429 0L2.884 6.288l-.846-.677Z"/>
            <path d="M20.677 4.117A1.996 1.996 0 0 0 20 4H4c-.225 0-.44.037-.642.105l.758.607L12 10.742 19.9 4.7l.777-.583Z"/>
        </svg>

        <DeleteSvg text={"Hello"} onclick={onclick_delete} class={false_prop_class} />

    </div>
    }
}
