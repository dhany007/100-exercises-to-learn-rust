// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, channel, sync_channel};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        let (tx, rx) = channel();

        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: tx
            })
            .map_err(|e| format!("failed to send insert command: {:?}", e))?;

        rx.recv().map_err(|e| {
            format!("failed to received: {:?}", e)
        })
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        let (tx, rx) = channel();

        self.sender 
            .send(Command::Get { 
                id,
                response_channel: tx
            })
            .map_err(|e| format!("failed to send get command: {:?}", e))?;

        rx.recv().map_err(|e| {
            format!("failed to received: {:?}", e)
        })
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id).cloned();
                let _ =response_channel.send(ticket);
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
