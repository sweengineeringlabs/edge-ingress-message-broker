//! Result type alias for inbound message consumption operations.

use super::consumer_error::ConsumerError;

/// Result type for [`MessageConsumer`](crate::MessageConsumer) operations.
pub type ConsumerResult<T> = Result<T, ConsumerError>;
