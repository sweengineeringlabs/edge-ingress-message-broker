//! `MessageConsumer` — ingress port for inbound message consumption.

use futures::future::BoxFuture;

use crate::api::{
    ConsumerBox, ConsumerError, ConsumerHealthResult, ConsumerResult, ConsumerStreamResult,
    HealthCheckFuture, SubscribeFuture,
};
use swe_edge_runtime_message_broker::MessageStream;

/// Subscribes to topics on an external message broker and receives messages.
///
/// Use the SAF factories to obtain a concrete implementation:
///
/// ```rust,ignore
/// // Requires feature = "in-memory"
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
    /// Returns a [`SubscribeFuture`] that resolves to a [`ConsumerStreamResult`].
    fn subscribe<'a>(&'a self, topic: &'a str) -> SubscribeFuture<'a>;

    /// Verify the consumer is connected and the broker is reachable.
    ///
    /// Returns a [`HealthCheckFuture`] that resolves to a [`ConsumerHealthResult`].
    fn health_check(&self) -> HealthCheckFuture<'_>;

    /// Subscribe and return the inner stream result as a [`ConsumerStreamResult`].
    ///
    /// Surfaces [`ConsumerStreamResult`] by name in the trait signature
    /// (required by SEA `api_no_orphan_types`).
    ///
    /// Default implementation delegates to [`subscribe`](Self::subscribe).
    fn subscribe_checked<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, ConsumerStreamResult> {
        self.subscribe(topic)
    }

    /// Run a health check and return the outcome as a [`ConsumerHealthResult`].
    ///
    /// Surfaces [`ConsumerHealthResult`] by name in the trait signature
    /// (required by SEA `api_no_orphan_types`).
    ///
    /// Default implementation delegates to [`health_check`](Self::health_check).
    fn health_check_result(&self) -> BoxFuture<'_, ConsumerHealthResult> {
        self.health_check()
    }

    /// Wrap `self` in an `Arc` to produce a [`ConsumerBox`].
    ///
    /// Only available when `Self: Sized + 'static`.
    fn into_box(self) -> ConsumerBox
    where
        Self: Sized + 'static,
    {
        std::sync::Arc::new(self)
    }

    /// Return a [`ConsumerError`] describing a disconnected state.
    ///
    /// Surfaces [`ConsumerError`] in the trait signature.
    fn disconnected_error(&self) -> ConsumerError {
        ConsumerError::Unavailable("consumer disconnected".into())
    }
}
