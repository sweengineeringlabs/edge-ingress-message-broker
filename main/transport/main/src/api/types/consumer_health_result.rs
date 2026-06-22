//! Type alias for the result of a consumer health-check call.

use crate::api::ConsumerError;

/// The result type returned by [`MessageConsumer::health_check`](crate::api::MessageConsumer::health_check).
///
/// `Ok(())` indicates the consumer is healthy; `Err(err)` indicates the
/// consumer is unavailable or the broker cannot be reached.
pub type ConsumerHealthResult = Result<(), ConsumerError>;
