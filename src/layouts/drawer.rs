use crate::Pages;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(DrawerPanel)]
pub fn drawer_panel() -> Html {

    html! {
        <div id={"drawer"} class="flex flex-col h-full min-w-fit w-1/2 sm:w-1/3 md:w-1/4 bg-nostr-dark z-60 gap-8  border-r-[1px] border-white justify-between items-start">
            <DrawerMenuLinks />
            <div class={"flex flex-row justify-between items-center gap-4 h-fit w-full p-4 border-nostr-light border-t-2"}>
                <p>{"Illuminodes"}</p>
                <img class="w-20 h-20 -m-4" src="/logo.png" />
            </div>
        </div>
    }
}

#[function_component(DrawerMenuLinks)]
pub fn drawer_menu() -> Html {
    let navigator = use_navigator();
    if navigator.is_none() {
        return html! {};
    }
    let navigator = navigator.unwrap();

    let route = use_route::<Pages>();
    if route.is_none() {
        return html! {};
    }
    let route = route.unwrap();

    let nav_clone = navigator.clone();
    let onclick_home = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        nav_clone.push(&Pages::Home);
    });

    let nav_clone = navigator.clone();
    let onclick_keys = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        nav_clone.push(&Pages::UserKeys);
    });

    let nav_clone = navigator.clone();
    let onclick_relays = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        nav_clone.push(&Pages::UserRelays);
    });

    let nav_clone = navigator.clone();
    let onclick_notes = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        nav_clone.push(&Pages::UserNotes);
    });

    let nav_clone = navigator.clone();
    let onclick_faq = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        nav_clone.push(&Pages::FAQ);
    });

    let unselected_class = "flex flex-row items-center gap-2 h-fit w-fit";
    let selected_class =
        "flex flex-row items-center gap-2 border-b-2 border-nostr-light p-2 h-fit w-fit";
    let svg_class = "stroke-white";

    html! {
        <div class="flex flex-col gap-8 text-lg m-4 my-8 mr-8">
            <div onclick={onclick_home} class={if route == Pages::Home {selected_class} else {unselected_class}}>
                <svg class={svg_class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m4 12 8-8 8 8M6 10.5V19a1 1 0 0 0 1 1h3v-3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v3h3a1 1 0 0 0 1-1v-8.5"/></svg>
                <a>{"Home"}</a>
            </div>
            <div onclick={onclick_keys} class={if route == Pages::UserKeys {selected_class} else {unselected_class}}>
                <svg class={svg_class} viewBox="-0.5 0 25 25" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.75 9.807a6 6 0 1 0-7.5 5.811v6.136a1.5 1.5 0 0 0 3 0v-6.136a6 6 0 0 0 4.5-5.811v0z" stroke="inherit" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M9.741 4.605a6 6 0 0 1 8.16 8.251l4.91 4.906a1.5 1.5 0 0 1-2.122 2.121l-4.907-4.906a5.972 5.972 0 0 1-2.172.766h-.044M6.75 10.557a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z" stroke="inherit" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M6.75 7.406V3a2.25 2.25 0 1 1 4.5 0v.766" stroke="inherit" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                <a >{"My Key"}</a>
            </div>
            <div onclick={onclick_relays} class={if route == Pages::UserRelays {selected_class} else {unselected_class}}>
                <svg class={format!("{} fill-white", svg_class)} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 318.524 318.524" ><path d="M203.309 86.015c0-16.756-9.488-31.33-23.369-38.654a27.612 27.612 0 0 0 6.455-17.764c0-15.294-12.442-27.737-27.737-27.737-15.294 0-27.737 12.443-27.737 27.737 0 6.964 2.586 13.331 6.84 18.207-13.432 7.459-22.546 21.788-22.546 38.211v23.269h88.094V86.015zM158.658 16.86c7.023 0 12.737 5.714 12.737 12.737s-5.714 12.736-12.737 12.736-12.737-5.713-12.737-12.736 5.714-12.737 12.737-12.737zm29.651 77.424h-58.094v-8.269c0-15.815 12.867-28.682 28.682-28.682h.73c15.815 0 28.682 12.867 28.682 28.682v8.269zM64.724 254.74a27.612 27.612 0 0 0 6.455-17.764c0-15.294-12.442-27.736-27.736-27.736s-27.737 12.442-27.737 27.736c0 6.964 2.586 13.331 6.84 18.208C9.114 262.642 0 276.971 0 293.394v23.27h88.094v-23.27c0-16.756-9.489-31.33-23.37-38.654zm-21.281-30.501c7.023 0 12.736 5.713 12.736 12.736s-5.713 12.737-12.736 12.737-12.737-5.714-12.737-12.737 5.714-12.736 12.737-12.736zm29.651 77.424H15v-8.27c0-15.815 12.866-28.681 28.681-28.681h.731c15.815 0 28.682 12.866 28.682 28.681v8.27zM295.154 254.74a27.612 27.612 0 0 0 6.455-17.764c0-15.294-12.442-27.736-27.736-27.736s-27.737 12.442-27.737 27.736c0 6.964 2.586 13.331 6.84 18.208-13.432 7.459-22.546 21.788-22.546 38.21v23.27h88.094v-23.27c-.001-16.756-9.489-31.33-23.37-38.654zm-21.281-30.501c7.022 0 12.736 5.713 12.736 12.736s-5.714 12.737-12.736 12.737c-7.023 0-12.737-5.714-12.737-12.737s5.714-12.736 12.737-12.736zm29.65 77.424H245.43v-8.27c0-15.815 12.866-28.681 28.682-28.681h.73c15.815 0 28.682 12.866 28.682 28.681v8.27h-.001zM166.762 134.949h-15v49.393l-52.333 52.333 10.606 10.607 49.227-49.227 49.226 49.227 10.608-10.607-52.334-52.333z"/></svg>
                <a >{"My Relays"}</a>
            </div>
            <div onclick={onclick_notes} class={if route == Pages::UserNotes {selected_class} else {unselected_class}}>
                <svg  class={svg_class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 11H4m15.5 5a.5.5 0 0 0 .5-.5V8a1 1 0 0 0-1-1h-3.75a1 1 0 0 1-.829-.44l-1.436-2.12a1 1 0 0 0-.828-.44H8a1 1 0 0 0-1 1M4 9v10a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1v-7a1 1 0 0 0-1-1h-3.75a1 1 0 0 1-.829-.44L9.985 8.44A1 1 0 0 0 9.157 8H5a1 1 0 0 0-1 1Z"/></svg>
                <a >{"My Notes"}</a>
            </div>
            <div onclick={onclick_faq} class={if route == Pages::FAQ {selected_class} else {unselected_class}}>
                <svg class={svg_class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.529 9.988a2.502 2.502 0 1 1 5 .191A2.441 2.441 0 0 1 12 12.582V14m-.01 3.008H12M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"/></svg>
                <a >{"FAQ"}</a>
            </div>
        </div>
    }
}
