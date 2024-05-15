use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
mod atoms;
mod js;
mod layouts;
mod relay_handler;
mod user_keys;
mod user_notes;
mod user_relays;

use crate::atoms::sections::{AppPage, AppSection, SectionTitle, SectionParagraph};
use crate::js::web_utils::init_service_worker;
use crate::layouts::main_panel::MainPanelLayout;
use crate::relay_handler::{bunker_connect::BunkerConnectExport, relay_pool::RelayPool};
use crate::user_keys::{
    key_context::{KeyContext, NostrKeysContext},
    key_details::NostrKeyDetails,
    key_export::NostrPublicKeyExport,
    key_page::NostrKeyPage,
};
use crate::user_notes::{note_context::UserNotesProvider, note_page::UserNotesPage};
use crate::user_relays::{relay_context::RelayContext, relay_page::UserRelaysPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Pages {
    #[at("/")]
    Home,
    #[at("/userKeys")]
    UserKeys,
    #[at("/userRelays")]
    UserRelays,
    #[at("/userNotes")]
    UserNotes,
    #[at("/FAQ")]
    FAQ,
}

#[function_component(App)]
fn app() -> Html {
    use_effect_with((), |_| {
        spawn_local(async {
            init_service_worker().await;
        });
        || {}
    });

    html! {
    <>
        <KeyContext >
            <RelayContext >
                <UserNotesProvider >
                <BrowserRouter>
                    <MainPanelLayout >
                            <Switch<Pages> render = { move |switch: Pages|{
                                match switch {
                                    Pages::Home => html! {
                                        <HomePage />
                                    },
                                    Pages::UserKeys => html! {
                                        <NostrKeyPage />
                                    },
                                    Pages::UserRelays => html! {
                                        <UserRelaysPage />
                                    },
                                    Pages::UserNotes => html! {
                                        <UserNotesPage />
                                    },
                                    Pages::FAQ => html! {
                                        <FAQPage />
                                    },
                                }
                            }}
                        />
                    </MainPanelLayout>
                </BrowserRouter>
                </UserNotesProvider>
            </RelayContext>
        </KeyContext>
    </>
    }
}

#[function_component(HomePage)]
fn home_page() -> Html {
    let key_context = use_context::<NostrKeysContext>();
    if key_context.is_none() {
        return html! {};
    }
    let key_context = key_context.unwrap();
    let key_exporter = match key_context.has_keys() {
        false => html! {
            <>
                <SectionParagraph text="No key found." />
                <SectionParagraph text="Please generate or import one." />
            </>
        },
        true => html! {
            <>
                    <SectionTitle title="Bunker Home" />
                    <NostrKeyDetails />
                    <div class="pl-8">
                        <BunkerConnectExport />
                    </div>
                    <NostrPublicKeyExport />
                    <RelayPool />
            </>
        },
    };
    html! {
            <AppPage >
                <AppSection >
                    {key_exporter}
                </AppSection>
            </AppPage>
    }
}

#[function_component(FAQPage)]
fn faq_page() -> Html {
    html! {
        <AppPage >
            <AppSection >
                <SectionTitle title="What is NIP-46?" />
                <SectionParagraph text={"NIP-46 helps keep your private keys safe when signing Nostr events."} />

                <SectionTitle title="How does it work?" />
                <SectionParagraph text={"It lets a Nostr client communicate securely with a remote signer like this one to request your signature."} />

                <SectionTitle title="How do I use it?" />
                <SectionParagraph text={
                    r#"
                        Add your key and some relays. You'll be able to see requests from 
                        Nostr clients that support NIP-46 on the home page.
                        "#
                } />

                <SectionTitle title={"Anything else to know?"} />
                <SectionParagraph text={"It's about keeping things secure without making it complicated."} />
            </AppSection>
        </AppPage>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
