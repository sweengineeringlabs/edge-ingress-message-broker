//! NATS-backed `NatsMessageConsumer` implementation.
#[cfg(feature = "nats")]
mod nats_message_consumer;
#[cfg(feature = "nats")]
pub(crate) use nats_message_consumer::NatsMessageConsumer;
