use crate::atoms::sections::{AppPage, AppSection, SectionParagraph, SectionTitle};
use crate::user_notes::note_list::UserNotesList;
use yew::prelude::*;

#[function_component(UserNotesPage)]
pub fn user_notes() -> Html {
    let title = "Your Saved Notes";
    let text = r#"
            Relays are not guaranteed to save your notes. 
            Keep a local copy of your important notes to prevent data loss.
        "#;

    html! {
        <AppPage>
            <AppSection >
                <SectionTitle {title} />
                <SectionParagraph {text} />
                <UserNotesList />
            </AppSection>
        </AppPage>
    }
}
