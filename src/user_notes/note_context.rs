use gloo::{console::log, utils::format::JsValueSerdeExt};
use js_sys::Object;
use nostro2::notes::SignedNote;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use yew::{platform::spawn_local, prelude::*};

use crate::js::indexed_db::{retrieve_all_from_store, IdbStores, add_to_store, delete_from_store};

pub enum UserNotesAction {
    InitNotes(Vec<SignedNote>),
    AddNote(SignedNote),
    RemoveNote(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserNotes {
    user_notes: Vec<SignedNote>,
}

impl UserNotes {
    pub fn get_notes(&self) -> Vec<SignedNote> {
        self.user_notes.clone()
    }
}

impl Reducible for UserNotes {
    type Action = UserNotesAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            UserNotesAction::InitNotes(notes) => {
                let mut note_array = self.user_notes.clone();
                note_array.extend(notes);
                Rc::new(UserNotes {
                    user_notes: note_array,
                })
            }
            UserNotesAction::AddNote(note) => {
                log!("Adding note to store", &format!("{}", note));
                let mut note_array = self.user_notes.clone();
                let note_js_value = JsValue::from_serde(&note).unwrap();
                note_array.push(note);
                spawn_local(async move {
                    log!("Adding note to store", &format!("{:?}", &note_js_value));
                    if let Err(e) = add_to_store(IdbStores::UserNotes, note_js_value).await {
                        log!("Error adding note to store: ", e);
                    }
                });
                Rc::new(UserNotes {
                    user_notes: note_array,
                })
            }
            UserNotesAction::RemoveNote(note_id) => {
                let mut note_array = self.user_notes.clone();
                note_array.retain(|x| x.get_id() != &note_id);
                spawn_local(async move {
                    if let Err(e) = delete_from_store(IdbStores::UserNotes, &note_id).await {
                        log!("Error removing note from store: ", e);
                    }
                });
                Rc::new(UserNotes {
                    user_notes: note_array,
                })
            }
        }
    }
}

pub type UserNotesContext = UseReducerHandle<UserNotes>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct UserNotesChildren {
    pub children: Children,
}

#[function_component(UserNotesProvider)]
pub fn note_context(props: &UserNotesChildren) -> Html {
    let ctx = use_reducer(|| UserNotes { user_notes: vec![] });

    let ctx_clone = ctx.clone();
    use_effect_with((), move |_| {
        let ctx = ctx_clone.clone();
        spawn_local(async move {
            if let Ok(note_list) = retrieve_all_from_store(IdbStores::UserNotes).await {
                let res = js_array_to_notes_list(note_list);
                if let Ok(notes) = res {
                    ctx.dispatch(UserNotesAction::InitNotes(notes.user_notes));
                }
            } else {
                log!("Error retrieving notes from store");
            }
        });
        || {}
    });

    html! {
        <ContextProvider<UserNotesContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<UserNotesContext>>
    }
}

fn js_array_to_notes_list(array_buffer: JsValue) -> Result<UserNotes, String> {
    let cast_vec = array_buffer.dyn_into::<js_sys::Array>().unwrap();
    let mut user_notes = vec![];
    for i in 0..cast_vec.length() {
        let note = cast_vec.get(i);
        let note = note.dyn_into::<Object>().unwrap();
        let user_note: SignedNote = note.into_serde().unwrap();
        user_notes.push(user_note);
    }
    Ok(UserNotes { user_notes })
}
