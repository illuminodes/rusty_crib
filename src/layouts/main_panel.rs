use crate::layouts::drawer::DrawerPanel;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MainPanelProps {
    pub children: Html,
}

#[function_component(MainPanelLayout)]
pub fn main_panel(props: &MainPanelProps) -> Html {
    html! {
        <div class="flex flex-col flex-1 pt-16">
            <MainPanelBanner />
            <div class="flex flex-col w-full p-4 sm:p-8 items-center">
                {props.children.clone()}
            </div>
        </div>
    }
}

use wasm_bindgen::JsCast;
#[function_component(MainPanelBanner)]
pub fn main_panel_banner() -> Html {
    let drawer_open = use_state(|| false);

    let drawer_open_clone = drawer_open.clone();
    let onclick = Callback::from(move |_| {
        drawer_open_clone.set(!(*drawer_open_clone));
    });

    let drawer_clone = drawer_open.clone();
    let onclick_close = Callback::from(move |e: MouseEvent| {
        let element = e.target().unwrap();
        let element: web_sys::Element = element.dyn_into().unwrap();
        let element_id = element.id();
        if element_id == "not_drawer" {
            drawer_clone.set(false);
        }
    });


    html! {
        <>
        <div class="fixed top-0 h-16 w-full text-white p-4 flex flex-row items-center gap-4 bg-black">
            <svg {onclick} 
                class="w-12 h-12 text-white"
                aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M5 7h14M5 12h14M5 17h14"/>
            </svg>

            <h1 class="text-xl font-semibold">{"Rusty CRIB"}</h1>
            <input type="checkbox" id="drawer-toggle" class="hidden peer" checked={*drawer_open} />
            <div id={"not_drawer"} onclick={onclick_close} class="fixed inset-0 z-20 transition-all duration-[420] transform -translate-x-full bg-transparent shadow-lg peer-checked:translate-x-0">
                <DrawerPanel />
            </div>
        </div>
        </>
    }
}
