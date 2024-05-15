use gloo_timers::callback::Timeout;
use yew::{prelude::*, virtual_dom::VNode};

pub fn error_handler(state_handler: UseStateHandle<VNode>, error: String) {
    state_handler.set(html! {
        <ErrorToast error={error}/>
    });
    let error_clone = state_handler.clone();
    let _ = Timeout::new(2100, move || {
        error_clone.set(html! {});
    })
    .forget();
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ErrorToastProps {
    pub error: String,
}

#[function_component(ErrorToast)]
pub fn error_toast(props: &ErrorToastProps) -> Html {
    let error = props.error.clone();
    html! {
    <div class="fixed bottom-0 right-0 p-8">
        <div class="bg-red text-white font-bold rounded-lg border shadow-lg p-4">
            <div class="flex flex-row">
                <div class="flex flex-col">
                    <div>{&*error}</div>
                </div>
            </div>
        </div>
    </div>
    }
}

pub fn action_handler(state_handler: UseStateHandle<VNode>, action: String) {
    state_handler.set(html! {
        <ActionBubble  {action} />
    });
    let success_clone = state_handler.clone();
    let _ = Timeout::new(420, move || {
        success_clone.set(html! {});
    })
    .forget();
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ActionBubbleProps {
    pub action: String,
}

#[function_component(ActionBubble)]
pub fn action_bubble(props: &ActionBubbleProps) -> Html {
    html! {
        <div class="fixed inset-0 flex w-full items-center flex-col-reverse">
            <div class="pb-16">
                <div class="w-fit h-fit px-4 py-2 font-bold bg-green border border-white rounded-lg">
                    <p>{&props.action}</p>
                </div>
            </div>
        </div>
    }
}
