//! Future type alias for async subscribe operations.

use futures::future::BoxFuture;

use crate::api::ConsumerStreamResult;

/// A pinned, heap-allocated future that resolves to a [`ConsumerStreamResult`].
///
/// Returned by [`MessageConsumer::subscribe`](crate::api::MessageConsumer::subscribe)
/// and [`TransportSvc::subscribe_to`](crate::saf::transport_svc).
pub type SubscribeFuture<'a> = BoxFuture<'a, ConsumerStreamResult>;
