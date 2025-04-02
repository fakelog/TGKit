mod client;
mod event_dispatcher;
mod utils;

pub use client::Client;
pub use event_dispatcher::EventDispatcher;
pub use tg_kit_core;

#[cfg(feature = "new-message-event-handler")]
pub extern crate new_message_event_handler;
