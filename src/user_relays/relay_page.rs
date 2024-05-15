use crate::atoms::sections::{AppPage, AppSection, SectionParagraph, SectionTitle};
use crate::user_relays::{new_relay::NewRelay, relay_list::RelayList};
use yew::prelude::*;

#[function_component(UserRelaysPage)]
pub fn user_relays() -> Html {
    html! {
        <AppPage>
            <AppSection>
                <SectionTitle title="Add Relays" />
                <SectionParagraph text={ r#"
                        Relays are used to send and receive messages on the Nostr network.
                        You can add a new relay by entering the relay's URL.
                    "# } />
                <NewRelay />
            </AppSection>
            <AppSection>
                <SectionTitle title="Your Relays" />
                <SectionParagraph text={ r#"
                        Here are the relays you have added to your bunker.
                        You can disable read or write access to a relay by clicking the appropriate button.
                    "# } />
                <RelayList />
            </AppSection>
        </AppPage>
    }
}
