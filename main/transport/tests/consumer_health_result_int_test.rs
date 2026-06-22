//! Integration tests — ConsumerHealthResult type alias.
//!
//! Verifies that `ConsumerHealthResult` resolves to `Result<(), ConsumerError>`
//! and that Ok(()) is the healthy outcome.

use swe_edge_ingress_message_broker_transport::{ConsumerError, ConsumerHealthResult};

/// @covers: api/types/consumer_health_result.rs::ConsumerHealthResult — Ok(()) is the healthy outcome.
#[test]
fn test_consumer_health_result_type_ok_unit_represents_healthy_happy() {
    let result: ConsumerHealthResult = Ok(());
    assert!(result.is_ok(), "Ok(()) must represent a healthy state");
}

/// @covers: api/types/consumer_health_result.rs::ConsumerHealthResult — Err carries ConsumerError.
#[test]
fn test_consumer_health_result_type_err_carries_consumer_error_error() {
    let result: ConsumerHealthResult = Err(ConsumerError::Unavailable("down".into()));
    assert!(result.is_err(), "Err must represent an unhealthy state");
    match result {
        Err(ConsumerError::Unavailable(msg)) => assert_eq!(msg, "down"),
        _ => panic!("expected ConsumerError::Unavailable"),
    }
}

/// @covers: api/types/consumer_health_result.rs::ConsumerHealthResult — usable as a fn return type.
#[test]
fn test_consumer_health_result_type_usable_as_return_type_edge() {
    fn ok_result() -> ConsumerHealthResult { Ok(()) }
    fn err_result() -> ConsumerHealthResult {
        Err(ConsumerError::Connection("refused".into()))
    }
    assert!(ok_result().is_ok());
    assert!(err_result().is_err());
}
