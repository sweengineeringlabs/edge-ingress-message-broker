//! Type alias for the result of a consumer subscribe call.

use swe_edge_runtime_message_broker::MessageStream;

use crate::api::ConsumerResult;

/// The result type returned when subscribing to a message topic.
///
/// `Ok(stream)` indicates a successful subscription; `Err(err)` indicates
/// the subscribe operation failed (e.g. broker unreachable, invalid topic).
pub type ConsumerStreamResult = ConsumerResult<MessageStream>;
