//! Future type alias for async health-check operations.

use futures::future::BoxFuture;

use crate::api::ConsumerError;

/// A pinned, heap-allocated future that resolves to a consumer health-check result.
///
/// Returned by [`MessageConsumer::health_check`](crate::api::MessageConsumer::health_check)
/// and [`TransportSvc::check_health`](crate::saf::transport_svc).
pub type HealthCheckFuture<'a> = BoxFuture<'a, Result<(), ConsumerError>>;
