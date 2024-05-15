use std::sync::Arc;

use async_channel::{unbounded, Sender};
use nostro2::{
    notes::SignedNote,
    relays::{NostrRelay, RelayEvents},
};
use serde_json::json;

use yew::platform::spawn_local;
use yew::{prelude::*, props};
use crate::user_relays::relay_context::UserRelay;

pub enum RelayAction {
    RelayMessage(RelayEvents),
    NewNote(SignedNote),
    ClearNote,
}

#[derive(Properties, Clone, PartialEq)]
pub struct NostrProps {
    pub last_note: Option<SignedNote>,
    pub send_note: Callback<SignedNote>,
    pub clear_note: Callback<()>,
}


#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RelayContextProps {
    pub children: Children,
    pub user_relay: UserRelay,
    pub user_pk: String,
}

pub struct RelayComponent {
    latest_note: Option<SignedNote>,
    sender_channel: Sender<SignedNote>,
    close_channel: Sender<()>,
    send_note_callback: Callback<SignedNote>,
    clear_note_callback: Callback<()>,
    children: Children,
}

impl Component for RelayComponent {
    type Message = RelayAction;
    type Properties = RelayContextProps;

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let props = self._build_props();
        html! {
            <>
                <ContextProvider<NostrProps> context={props}>
                    {self.children.clone()}
                </ContextProvider<NostrProps>>
            </>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        let relay_url = ctx.props().user_relay.url.clone();
        let user_pk = ctx.props().user_pk.clone();
        let message_cb = ctx.link().callback(RelayAction::RelayMessage);
        let (sender_channel, close_channel) = Self::read_relay(message_cb, relay_url, user_pk);
        let send_note_callback = ctx.link().callback(RelayAction::NewNote);
        let clear_note_callback = ctx.link().callback(move |_| RelayAction::ClearNote);
        let children = ctx.props().children.clone();

        Self {
            latest_note: None,
            sender_channel,
            close_channel,
            send_note_callback,
            clear_note_callback,
            children,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let is_relay_read = ctx.props().user_relay.read;
        let is_relay_write = ctx.props().user_relay.write;
        match msg {
            RelayAction::RelayMessage(msg) => match &msg {
                RelayEvents::EVENT(_, _, signed_note) => {
                    if is_relay_read {
                        self.latest_note = Some(signed_note.clone());
                    }
                    true
                }
                _ => {
                    false
                }
            },
            RelayAction::NewNote(note) => {
                if is_relay_write {
                    self.send_nostr_note(note);
                    self.latest_note = None;
                }
                true
            }
            RelayAction::ClearNote => {
                self.latest_note = None;
                true
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.close_ws();
    }
}

impl RelayComponent {
    fn read_relay(
        note_cb: Callback<RelayEvents>,
        relay_url: String,
        user_pk: String,
    ) -> (Sender<SignedNote>, Sender<()>) {
        let (note_tx, note_rx) = unbounded::<SignedNote>();
        let (close_tx, close_rx) = unbounded::<()>();

        spawn_local(async move {
            let relay = NostrRelay::new(&relay_url).await;
            if let Err(_) = relay {
                return;
            };
            let relay = relay.unwrap();
            let relay_arc = Arc::new(relay);

            let sender_relay = relay_arc.clone();
            spawn_local(async move {
                while let Ok(note) = note_rx.recv().await {
                    sender_relay.send_note(note).await.unwrap();
                }
            });

            let reader_relay = relay_arc.clone();
            spawn_local(async move {
                let filter = json!({
                    "kinds": [24133],
                    "#p": [user_pk],
                });

                reader_relay.subscribe(filter).await.unwrap();

                while let Ok(event) = reader_relay.read_relay_events().await {
                    note_cb.emit(event);
                }
            });
            let close_relay = relay_arc.clone();
            spawn_local(async move {
                while let Ok(_) = close_rx.recv().await {
                    close_relay.close().await;
                }
            });
        });
        (note_tx, close_tx)
    }

    pub fn _build_props(&self) -> NostrProps {
        props!(NostrProps {
            last_note: self.latest_note.clone(),
            send_note: self.send_note_callback.clone(),
            clear_note: self.clear_note_callback.clone(),
        })
    }

    fn send_nostr_note(&self, signed_note: SignedNote) {
        self.sender_channel.try_send(signed_note).unwrap();
    }

    fn close_ws(&self) {
        self.close_channel.try_send(()).unwrap();
    }
}
