//! Core implementations — pub(crate) only.
pub(crate) mod default;
pub(crate) mod nats;

#[cfg(feature = "in-memory")]
pub(crate) use default::DefaultMessageConsumer;
#[cfg(feature = "nats")]
pub(crate) use nats::NatsMessageConsumer;
