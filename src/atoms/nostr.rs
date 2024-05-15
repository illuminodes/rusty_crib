use chrono::{DateTime, Local, TimeZone};
use nostro2::notes::{Note, SignedNote};
use yew::prelude::*;

use crate::{
    user_keys::key_context::NostrKeys,
    user_notes::note_context::{UserNotes, UserNotesAction, UserNotesContext},
};

#[derive(Clone, Properties, PartialEq)]
pub struct SignedNoteProps {
    pub signed_note: SignedNote,
}

#[function_component(SignedNoteDetail)]
pub fn signed_note_detail(props: &SignedNoteProps) -> Html {
    let note = &props.signed_note;
    let local_utc_offset = Local
        .timestamp_opt(0, 0)
        .unwrap()
        .offset()
        .local_minus_utc();
    let note_created_at = note.get_created_at() as i32 + local_utc_offset;
    let created_date = DateTime::from_timestamp(note_created_at as i64, 0)
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S");

    html! {
        <div class="flex flex-col p-4 gap-4 border border-nostr rounded-lg text-sm bg-black">
            <p>{note.get_content()}</p>
            <div class="flex flex-row gap-4">
                <div class="flex flex-row gap-2">
                    <p class="font-bold">{"Kind:"}</p>
                    <p>{note.get_kind()}</p>
                </div>
                <div class="flex flex-row gap-2">
                    <p class="font-bold">{"Created At:"}</p>
                    <p>{format!("{}", created_date)}</p>
                </div>
            </div>
            {for note.get_tags().iter().map(|tag| {
                html! {
                    <p>{format!("Tag: {:?}", tag)}</p>
                }
            })}
            <div class="flex flex-row gap-4">
                <div class="flex flex-row gap-2">
                    <p class="font-bold">{"ID:"}</p>
                    <p>{&note.get_id()[..12]}</p>
                </div>
                <div class="flex flex-row gap-2">
                    <p class="font-bold">{"Sig:"}</p>
                    <p>{&note.get_sig()[..12]}</p>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct NoteProps {
    pub note: Note,
}

#[function_component(NoteDetail)]
pub fn note_detail(props: &NoteProps) -> Html {
    let note = &props.note;
    let local_utc_offset = Local
        .timestamp_opt(0, 0)
        .unwrap()
        .offset()
        .local_minus_utc();
    let note_created_at = note.created_at as i32 + local_utc_offset;
    let created_date = DateTime::from_timestamp(note_created_at as i64, 0)
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S");

    html! {
        <div class="flex flex-col p-4 gap-4 border border-purple-700 rounded-lg">
            <p>{format!("Content: {}", note.content)}</p>
            <div class="flex flex-row gap-4">
                <p>{format!("Kind: {}", note.kind)}</p>
                <p>{format!("Created At: {}", created_date)}</p>
            </div>
            {for note.tags.iter().map(|tag| {
                html! {
                    <p>{format!("Tag: {:?}", tag)}</p>
                }
            })}
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct CommandProps {
    pub latest: SignedNote,
    pub user_keys: UseReducerHandle<NostrKeys>,
    pub send_note: Callback<SignedNote>,
    pub clear_note: Callback<()>,
    pub save_note: UseReducerHandle<UserNotes>,
    pub request_id: String,
    pub request_note: Option<Note>,
}

#[function_component(ConnectionRequest)]
pub fn connection_request(props: &CommandProps) -> Html {
    let user_keys = props.user_keys.clone();
    let latest = props.latest.clone();
    let send_note = props.send_note.clone();
    let clear_note = props.clear_note.clone();
    let command = user_keys.clone().extract_nip46_command(&latest);

    let command_clone = command.clone();
    let onclick = Callback::from(move |_| {
        let command_response = user_keys.respond_to_nip46_command(command_clone.clone());
        send_note.emit(command_response);
    });

    let onclick_reject = Callback::from(move |_| {
        clear_note.emit(());
    });
    let connection_id = props.request_id.clone();

    html! {
        <div class="flex flex-col gap-4">
            <p>{"New Connection Request!"}</p>
            <p>{format!("ID: {}", connection_id)}</p>
            <div class="flex flex-row gap-4">
                <button {onclick} >{"Connect"}</button>
                <button onclick={onclick_reject} class="bg-red-700" >{"Reject"}</button>
            </div>
        </div>
    }
}

#[function_component(SignatureRequest)]
pub fn signature_request(props: &CommandProps) -> Html {
    let note_ctx = use_context::<UserNotesContext>();
    let user_keys = props.user_keys.clone();
    let latest = props.latest.clone();
    let send_note = props.send_note.clone();
    let clear_note = props.clear_note.clone();

    let send_note_clone = send_note.clone();
    let command = user_keys.clone().extract_nip46_command(&latest);

    let command_clone = command.clone();
    let user_keys_clone = user_keys.clone();
    let onclick = Callback::from(move |_| {
        let command_response = user_keys_clone.respond_to_nip46_command(command_clone.clone());
        send_note_clone.emit(command_response);
    });

    let command_clone = command.clone();
    let send_note_clone = send_note.clone();
    let save_note = note_ctx.clone().unwrap();
    let note_req = props.request_note.clone().unwrap();
    let onclick_send_and_save = Callback::from(move |_| {
        let signed_note = user_keys.sign_note(note_req.clone());
        let command_response = user_keys.respond_to_nip46_command(command_clone.clone());
        send_note_clone.emit(command_response);
        save_note.dispatch(UserNotesAction::AddNote(signed_note));
    });

    let onclick_reject = Callback::from(move |_| {
        clear_note.emit(());
    });

    let connection_id = props.request_id.clone();
    let note_req = props.request_note.clone().unwrap();

    html! {
        <div class="flex flex-col gap-4">
            <p>{"New Signature Request!"}</p>
            <p>{format!("ID: {}", connection_id)}</p>
            <NoteDetail note={note_req.clone()} />
            <div class="flex flex-row gap-4">
                <button {onclick} >{"Sign"}</button>
                <button onclick={onclick_send_and_save} >{"Sign and Save"}</button>
                <button onclick={onclick_reject} class="bg-red-700" >{"Reject"}</button>
            </div>
        </div>
    }
}
