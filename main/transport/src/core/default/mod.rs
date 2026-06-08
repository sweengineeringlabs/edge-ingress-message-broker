//! In-memory `DefaultMessageConsumer` implementation.
#[cfg(feature = "in-memory")]
mod default_message_consumer;
#[cfg(feature = "in-memory")]
pub(crate) use default_message_consumer::DefaultMessageConsumer;
