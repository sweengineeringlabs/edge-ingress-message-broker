//! Integration tests — ConsumerResult type alias.

use swe_edge_ingress_message_broker_transport::{ConsumerError, ConsumerResult};

/// @covers: ConsumerResult — Ok variant wraps the value
#[test]
fn test_consumer_result_ok_wraps_value() {
    let r: ConsumerResult<u32> = Ok(42);
    assert!(r.is_ok());
}

/// @covers: ConsumerResult — Err variant wraps ConsumerError
#[test]
fn test_consumer_result_err_wraps_consumer_error() {
    let r: ConsumerResult<()> = Err(ConsumerError::Unavailable("down".into()));
    assert!(r.is_err());
}
