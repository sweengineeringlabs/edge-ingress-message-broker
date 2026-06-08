//! `MessageConsumer` — ingress port for inbound message consumption.

use futures::future::BoxFuture;
use swe_edge_runtime_message_broker::MessageStream;

use crate::api::error::ConsumerResult;

/// Subscribes to topics on an external message broker and receives messages.
///
/// Use the SAF factories to obtain a concrete implementation:
///
/// ```rust,no_run
/// use swe_edge_ingress_message_broker_transport::TransportSvc;
///
/// let consumer = TransportSvc::default_consumer();
/// ```
///
/// # Feature flags
///
/// | Feature    | Backend                                  |
/// |------------|------------------------------------------|
/// | `in-memory`| `InMemoryMessageBroker` (tokio broadcast) |
/// | `nats`     | `NatsMessageBroker` (async-nats)         |
pub trait MessageConsumer: Send + Sync {
    /// Subscribe to `topic` and return a lazy stream of incoming messages.
    ///
    /// The stream yields `Ok(Message)` for each received message and
    /// `Err(ConsumerError::StreamLagged)` when the internal buffer overflows.
    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, ConsumerResult<MessageStream>>;

    /// Verify the consumer is connected and the broker is reachable.
    fn health_check(&self) -> BoxFuture<'_, ConsumerResult<()>>;
}
