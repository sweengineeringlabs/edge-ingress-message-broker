//! Integration tests — ConsumerError conversion and display.

use swe_edge_ingress_message_broker_transport::ConsumerError;

/// @covers: ConsumerError — Unavailable displays message
#[test]
fn test_consumer_error_unavailable_displays_message() {
    let e = ConsumerError::Unavailable("broker down".into());
    assert!(e.to_string().contains("broker down"));
}

/// @covers: ConsumerError — Connection displays message
#[test]
fn test_consumer_error_connection_displays_message() {
    let e = ConsumerError::Connection("refused".into());
    assert!(e.to_string().contains("refused"));
}

/// @covers: ConsumerError::Subscribe — displays topic and reason
#[test]
fn test_consumer_error_subscribe_displays_topic_and_reason() {
    let e = ConsumerError::Subscribe {
        topic: "orders".into(),
        reason: "timeout".into(),
    };
    let s = e.to_string();
    assert!(s.contains("orders"), "expected topic in error: {s}");
    assert!(s.contains("timeout"), "expected reason in error: {s}");
}

/// @covers: ConsumerError::StreamLagged — displays count
#[test]
fn test_consumer_error_stream_lagged_displays_count() {
    let e = ConsumerError::StreamLagged(42);
    assert!(e.to_string().contains("42"));
}
