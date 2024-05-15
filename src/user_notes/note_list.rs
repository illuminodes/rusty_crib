use crate::atoms::{
    buttons::{CopyOption, DeleteSvg},
    nostr::{SignedNoteDetail, SignedNoteProps},
};
use crate::js::bindings::clipboardCopy;
use crate::js::web_utils::confirm_user_action;
use crate::user_notes::note_context::{UserNotesAction, UserNotesContext};
use chrono::{DateTime, Local, TimeZone};
use gloo::console::error;
use gloo_timers::callback::Timeout;
use nostro2::notes::SignedNote;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component(UserNotesList)]
pub fn notes_list() -> Html {
    let notes_list_ctx = use_context::<UserNotesContext>();

    let notes_list: UseStateHandle<Vec<SignedNote>> = use_state(|| vec![]);

    let notes_list_clone = notes_list.clone();
    use_effect_with(notes_list_ctx.clone(), move |deps| {
        let note_ctx = deps.as_ref().unwrap();
        let notes = note_ctx.get_notes();
        notes_list_clone.set(notes);
        move || {
            notes_list_clone.set(vec![]);
        }
    });

    if notes_list.is_empty() {
        return html! {
        <div>
            <p>{"No notes found"}</p>
        </div>
        };
    }
    html! {
        <ul class="flex flex-col gap-4 sm:gap-8">
            {for notes_list.iter().map(|signed_note| {
                html! {
                    <UserNotesListOptions signed_note={signed_note.clone()} />
                }
            })}
        </ul>
    }
}

#[function_component(UserNotesListOptions)]
pub fn notes_list_options(props: &SignedNoteProps) -> Html {
    let note = &props.signed_note;
    let note_id = &note.get_id()[..8];
    let local_utc_offset = Local
        .timestamp_opt(0, 0)
        .unwrap()
        .offset()
        .local_minus_utc();
    let note_created_at = note.get_created_at() as i32 + local_utc_offset;
    let created_date = DateTime::from_timestamp(note_created_at as i64, 0)
        .unwrap()
        .format("%Y-%m-%d");

    let note_ctx = use_context::<UserNotesContext>().clone().unwrap();
    let db_note_id = note.get_id().to_string();
    let onclick_delete = Callback::from(move |_| {
        let confirm = confirm_user_action("Are you sure you want to delete this note?");
        match confirm {
            Ok(true) => {
                note_ctx.dispatch(UserNotesAction::RemoveNote(db_note_id.clone()));
            }
            Ok(false) => {
                error!("User cancelled delete action");
            }
            Err(e) => {
                error!("Error confirming delete action: {:?}", e);
            }
        }
    });

    let note_string = note.to_string();
    let clicked_state = use_state(|| false);
    let clicked_clone = clicked_state.clone();
    let onclick_copy_note = Callback::from(move |_| {
        let note_string = note_string.clone();
        clicked_clone.set(true);
        let clicked_second_clone = clicked_clone.clone();
        spawn_local(async move {
            let _ = clipboardCopy(&note_string).await;
            let _ = Timeout::new(210, move || {
                clicked_second_clone.set(false);
            })
            .forget();
        });
    });

    let note_details_state = use_state(|| false);
    let note_details_clone = note_details_state.clone();

    let onclick_show_details = Callback::from(move |_| {
        note_details_clone.set(!(*note_details_clone));
    });

    let showing_class = if *note_details_state {
        "w-10 h-10 text-white border-4 border-nostr rounded-full"
    } else {
        "w-10 h-10 text-white border-4 border-white rounded-full -rotate-90"
    };


    let text = "";
    html! {
    <li class={"flex flex-col gap-4"}>
        <div class="flex flex-row justify-between w-full sm:gap-8 items-center font-bold text-sm">
            <div class="flex flex-row gap-4">
                <p>{note_id}</p>
                <p>{created_date.to_string()}</p>
            </div>
            <div class={"justify-self-end flex flex-row gap-4 items-center"}>
                <svg onclick={onclick_show_details} class={showing_class} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 9-7 7-7-7"/>
                </svg>
                <CopyOption {text} onclick={onclick_copy_note} class={""}/>
                <DeleteSvg {text} onclick={onclick_delete} class="text-white bg-red rounded-full border-4 border-white"/>
            </div>
        </div>
        {if *note_details_state {
            html! {
                <SignedNoteDetail signed_note={note.clone()} />
            }
        } else {
            html! {}
        }}
    </li>
    }
}
