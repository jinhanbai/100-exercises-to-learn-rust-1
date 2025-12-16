//! Bounded Channel Implementation for Ticket Store
//!
//! This module implements a client-server architecture using bounded channels for command
//! communication and unbounded channels for responses.
//!
//! ## Channel Design
//!
//! ### Command Channel (Bounded)
//! The main communication channel between clients and the server is **bounded**:
//! - **Purpose**: Provides backpressure - if the server is slow, clients wait instead of
//!   creating unbounded memory growth
//! - **Prevents**: Memory exhaustion from too many pending commands
//! - **Long-lived**: Shared across all requests from all clients
//!
//! ```
//! Command Channel (Bounded):
//! ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
//! │ Client1 │────▶│         │     │         │     │         │
//! └─────────┘     │  Queue  │────▶│ Server  │
//! ┌─────────┐     │ (max N) │     │         │
//! │ Client2 │────▶│         │     └─────────┘
//! └─────────┘     └─────────┘
//! ┌─────────┐     (If full, clients wait)
//! │ Client3 │────▶
//! └─────────┘
//! ```
//!
//! ### Response Channels (Unbounded)
//! Each request creates a temporary, one-time-use response channel that is **unbounded**:
//! - **Purpose**: Return a single value from server to client
//! - **Short-lived**: Created per request, used once, then dropped
//! - **One-to-one**: One sender, one receiver (no accumulation)
//! - **Synchronous**: Client waits for response, so no queue needed
//!
//! ```
//! Response Channels (Unbounded, per request):
//! Request 1: Client ←─── Response Channel 1 ───→ Server (one message)
//! Request 2: Client ←─── Response Channel 2 ───→ Server (one message)
//! Request 3: Client ←─── Response Channel 3 ───→ Server (one message)
//! (Each channel is dropped after use)
//! ```
//!
//! ## Why This Design?
//!
//! - **Command channel bounded**: Limits pending work, provides backpressure, prevents
//!   memory exhaustion from too many queued commands
//! - **Response channels unbounded**: Simpler, no accumulation risk (one message per
//!   channel), client already waiting synchronously
//! ## Real- World Restaurant Analogy:
//! * Command channel (bounded): like a restaurant with limited tables — if full, customers wait
//! * Response channels (unbounded): like a waiter bringing your order — one item, delivered immediately, no queue needed

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SendError};
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, SendError<Command>> {
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        
        let command = Command::Insert {
            draft,
            response_channel: response_sender,
        };
        
        self.sender.send(command)?;  // Can block if channel is full
        Ok(response_receiver.recv().expect("Server didn't respond"))
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, SendError<Command>> {
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        
        let command = Command::Get {
            id,
            response_channel: response_sender,
        };
        
        self.sender.send(command)?;  // Can block if channel is full
        Ok(response_receiver.recv().expect("Server didn't respond"))
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { 
        sender,
    }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
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
                let ticket = store.get(id);
                let _ = response_channel.send(ticket);
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
