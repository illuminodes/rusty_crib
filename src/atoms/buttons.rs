use crate::atoms::toasts::action_handler;
use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AnimatedButtonProps {
    pub button_text: String,
    pub onclick_text: String,
    pub onclick: Callback<()>,
}

#[function_component(AnimatedButton)]
pub fn animated_button(props: &AnimatedButtonProps) -> Html {
    let normal_text = props.button_text.clone();
    let button_text = use_state(|| normal_text);

    let button_text_clone = button_text.clone();
    let onclick_text = props.onclick_text.clone();
    let onclick_clone = props.onclick.clone();
    let normal_text_clone = props.button_text.clone();
    let onclick = Callback::from(move |_| {
        onclick_clone.emit(());
        button_text_clone.set(onclick_text.clone());
        let button_text_clone = button_text_clone.clone();
        let normal_text = normal_text_clone.clone();
        let _ = Timeout::new(420, move || {
            button_text_clone.set(normal_text);
        })
        .forget();
    });

    html! {
        <button {onclick} >{&*button_text}</button>
    }
}

#[function_component(LoadingSpinner)]
pub fn loading_spinner() -> Html {
    html! {
    <div class="relative z-[-10]">
        <svg aria-hidden="true" class="w-12 h-12 text-white animate-spin fill-nostr" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
            <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
        </svg>
        <span class="sr-only">{"Loading..."}</span>
    </div>
    }
}

const SVG_CLASS: &str = r#"
border-4 border-white rounded-full
"#;

const SVG_CLASS_CLICKED: &str = r#"
text-nostr border-4 border-nostr rounded-full 
"#;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SvgProps {
    pub text: String,
    pub class: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(CopyOption)]
pub fn copy_svg(props: &SvgProps) -> Html {
    let cb = props.onclick.clone();

    let clicked_state = use_state(|| false);
    let bubble_state = use_state(|| html! {});

    let onclick = {
        let clicked_state = clicked_state.clone();
        let bubble_state = bubble_state.clone();
        Callback::from(move |e: MouseEvent| {
            clicked_state.set(true);
            cb.emit(e);
            let bubble_state = bubble_state.clone();
            let clicked_state = clicked_state.clone();
            let _ = action_handler(bubble_state, "Copied".to_string());
            let _ = Timeout::new(210, move || {
                clicked_state.set(false);
            })
            .forget();
        })
    };
    let class = if *clicked_state {
        SVG_CLASS_CLICKED
    } else {
        SVG_CLASS
    };
    let class = format!("{} {}", class, props.class);
    html! {
    <>
        {(*bubble_state).clone()}
        <div class="flex flex-row gap-4 sm:gap-8 items-center">
            if &props.text != "" {
                <p class="text-nostr-light text-base font-bold">{&props.text}</p>
            }
            <svg {onclick} {class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linejoin="round" stroke-width="2" d="M9 8v3a1 1 0 0 1-1 1H5m11 4h2a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1h-7a1 1 0 0 0-1 1v1m4 3v10a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1v-7.13a1 1 0 0 1 .24-.65L7.7 8.35A1 1 0 0 1 8.46 8H13a1 1 0 0 1 1 1Z"/>
            </svg>
        </div>
    </>
    }
}

#[function_component(DeleteSvg)]
pub fn delete_svg(props: &SvgProps) -> Html {
    let onclick = props.onclick.clone();
    let class = props.class.clone();
    html! {
    <svg {onclick} {class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24">
        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18 17.94 6M18 18 6.06 6"/>
    </svg>
    }
}

const INPUT_CLASS: &str = r#"
bg-transparent border border-white p-2 sm:p-4 rounded-md text-white sm:text-sm md:text-base
placeholder-white placeholder:bg-transparent placeholder:text-xs sm:placeholder:text-sm md:placeholder:text-base
focus:outline-none focus:border-nostr focus:ring-2 focus:ring-nostr focus:ring-opacity-50
"#;

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let input_type = props.input_type.clone();
    let extra_input_class = props.extra_class.clone();
    let input_class = format!("{} {}", INPUT_CLASS, extra_input_class);
    let placeholder = props.placeholder.clone();
    let onchange = props.onchange.clone();
    html! {
    <input
        type={input_type}
        class={input_class}
        placeholder={placeholder} {onchange} />
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextInputProps {
    pub input_type: String,
    pub extra_class: String,
    pub placeholder: String,
    pub onchange: Callback<Event>,
}
