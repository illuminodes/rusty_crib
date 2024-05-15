use crate::{
    atoms::{
        buttons::LoadingSpinner,
        nostr::{CommandProps, ConnectionRequest, SignatureRequest},
    },
    relay_handler::relay_component::NostrProps,
    user_keys::key_context::NostrKeysContext,
    user_notes::note_context::UserNotesContext,
};
use nostro2::nips::nip_46::Nip46Commands;
use yew::prelude::*;

#[function_component(Nip46Notifier)]
pub fn nip46_notifier() -> Html {
    let user_keys = use_context::<NostrKeysContext>();
    let relay_ctx = use_context::<NostrProps>();
    let note_ctx = use_context::<UserNotesContext>();
    if relay_ctx.is_none() || user_keys.is_none() || note_ctx.is_none() {
        return html! {};
    };
    let latest_note_in_relay = relay_ctx.clone().unwrap().last_note.clone();
    if latest_note_in_relay.is_none() {
        return html! {
            <LoadingSpinner />
        };
    }
    let send_note = relay_ctx.clone().unwrap().send_note.clone();
    let clear_note = relay_ctx.clone().unwrap().clear_note.clone();
    let save_note = note_ctx.clone().unwrap();
    let latest = latest_note_in_relay.unwrap();
    let user_keys = user_keys.clone().unwrap();
    let command = user_keys.extract_nip46_command(&latest);
    let mut command_props = CommandProps {
        latest,
        user_keys,
        send_note,
        clear_note,
        save_note,
        request_id: String::new(),
        request_note: None,
    };

    let command_html = match command {
        Nip46Commands::Connect(_, id) => {
            command_props.request_id = id;
            html! {
                <ConnectionRequest ..command_props.clone() />
            }
        }
        Nip46Commands::Ping(_, id) => {
            command_props.request_id = id;
            html! {
                <ConnectionRequest ..command_props.clone() />
            }
        }
        Nip46Commands::SignEvent(_, id, note) => {
            command_props.request_id = id;
            command_props.request_note = Some(note);
            html! {
                <SignatureRequest ..command_props.clone() />
            }
        }
    };

    return html! {
        <div class="flex flex-col gap-4">
            {command_html}
        </div>
    };
}
