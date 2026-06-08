//! Marker trait for NATS-backed message consumers.
//!
//! Interface counterpart for `core/nats/nats_message_consumer`.

/// Marker supertrait for [`MessageConsumer`](crate::api::traits::MessageConsumer)
/// implementations backed by a NATS server.
///
/// Implemented automatically by any consumer created via [`crate::TransportSvc::nats_consumer`].
pub trait NatsMessageConsumer: crate::api::traits::MessageConsumer {}
