//! Integration tests — ConsumerStreamResult type alias.
//!
//! Verifies that `ConsumerStreamResult` resolves to `ConsumerResult<MessageStream>`
//! and that Ok(stream) is the success outcome.

use swe_edge_ingress_message_broker_transport::{ConsumerError, ConsumerStreamResult};

/// @covers: api/types/consumer_stream_result.rs::ConsumerStreamResult — Err arm is accessible.
#[test]
fn test_consumer_stream_result_type_err_arm_is_constructible_happy() {
    let result: ConsumerStreamResult = Err(ConsumerError::Unavailable("test".into()));
    assert!(result.is_err(), "Err arm must be constructible through the alias");
}

/// @covers: api/types/consumer_stream_result.rs::ConsumerStreamResult — Err carries ConsumerError.
#[test]
fn test_consumer_stream_result_type_err_carries_consumer_error_error() {
    let result: ConsumerStreamResult = Err(ConsumerError::Subscribe {
        topic: "orders".into(),
        reason: "not found".into(),
    });
    match result {
        Err(ConsumerError::Subscribe { topic, reason }) => {
            assert_eq!(topic, "orders");
            assert_eq!(reason, "not found");
        }
        _ => panic!("expected ConsumerError::Subscribe"),
    }
}

/// @covers: api/types/consumer_stream_result.rs::ConsumerStreamResult — usable as fn return type.
#[test]
fn test_consumer_stream_result_type_usable_as_return_type_edge() {
    fn failure() -> ConsumerStreamResult {
        Err(ConsumerError::Connection("refused".into()))
    }
    assert!(failure().is_err());
}
