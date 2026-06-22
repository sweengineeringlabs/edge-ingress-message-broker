//! Integration tests — SAF message consumer service constants.
//!
//! Verifies that `TOPIC_SEPARATOR` is the standard dot character used in
//! NATS-style hierarchical topic names (e.g. "orders.created").

use swe_edge_ingress_message_broker_transport::TOPIC_SEPARATOR;

/// @covers: saf/message_consumer_svc.rs::TOPIC_SEPARATOR — is the dot character.
#[test]
fn test_topic_separator_const_is_dot_char_happy() {
    assert_eq!(TOPIC_SEPARATOR, '.', "TOPIC_SEPARATOR must be the '.' character");
}

/// @covers: saf/message_consumer_svc.rs::TOPIC_SEPARATOR — splits topic segments correctly.
#[test]
fn test_topic_separator_const_splits_topic_correctly_error() {
    // "error" scenario: a wrong separator would silently corrupt topic routing.
    let topic = "orders.created.v2";
    let parts: Vec<&str> = topic.split(TOPIC_SEPARATOR).collect();
    assert_eq!(parts, vec!["orders", "created", "v2"]);
}

/// @covers: saf/message_consumer_svc.rs::TOPIC_SEPARATOR — joins segments back correctly.
#[test]
fn test_topic_separator_const_joins_segments_correctly_edge() {
    let segments = ["events", "payment", "confirmed"];
    let topic: String = segments.join(&TOPIC_SEPARATOR.to_string());
    assert_eq!(topic, "events.payment.confirmed");
}
