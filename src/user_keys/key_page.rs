use crate::atoms::sections::{AppPage, AppSection, SectionParagraph, SectionTitle};
use crate::user_keys::{
    key_context::NostrKeysContext,
    key_export::{NostrPrivateKeyExport, NostrPublicKeyExport},
    key_generator::{NostrKeyDelete, NostrKeyGen},
    key_import::{MnemonicKeyImport, StringKeyImport},
};
use yew::prelude::*;

#[function_component(NostrKeyPage)]
pub fn nostr_key_page() -> Html {
    let key_context = use_context::<NostrKeysContext>();
    if key_context.is_none() {
        return html! {};
    }
    let key_context = key_context.unwrap();
    let page_body = match key_context.has_keys() {
        false => html! {
        <>
            <AppSection>
                <SectionTitle title="Import Your Keys" />
                <SectionParagraph text="You can import your keys using a hex or nsec string." />
                <StringKeyImport />
                <SectionParagraph text="You can also import your keys using a mnemonic seed phrase." />
                <MnemonicKeyImport />
            </AppSection>
            <AppSection>
                <SectionTitle title="Generate Nostr Keys" />
                <SectionParagraph text={ r#"
                        Generate a key pair to use your bunker and interact with the Nostr network.
                        Please make a backup of your private key as soon as possible. 
                    "# } />
                <SectionParagraph text={ r#"
                        Warning: If you lose your private keys, nobody will be able to recover them for you.
                    "# } />
                <NostrKeyGen />
            </AppSection>
        </>
        },
        true => html! {
        <>
            <AppSection>
                <SectionTitle title="Your Public Keys" />
                <SectionParagraph text={ r#"
                        Some Nostr clients may require your public key to interact with them.
                        You can export your public key using hex or npub formats.
                    "# } />
                <NostrPublicKeyExport />
            </AppSection>
            <AppSection>
                <SectionTitle title="Your Private Keys" />
                <SectionParagraph text={ r#"
                        Your private keys are used to sign transactions and control your Nostr identity.
                        It is crucial to keep your private keys safe and secure. Be wary of pasting them into untrusted websites.
                        We recommend exporting your private key to a physical backup using a seed phrase.
                    "# } />
                <NostrPrivateKeyExport />
            </AppSection>
            <AppSection>
                <SectionTitle title="Deleting Your Keys" />
                <SectionParagraph text={ r#"
                        Please make sure you have a backup of your keys before deleting them.
                        WARNING: Deleting your keys before backing them up 
                        will make it impossible to recover your account.
                    "# } />
                <div class="pl-8">
                    <NostrKeyDelete />
                </div>
            </AppSection>
        </>
        },
    };
    html! {
        <AppPage >
            {page_body}
        </AppPage>
    }
}
