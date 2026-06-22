//! Integration tests — ConsumerFuture kind constant.
//!
//! Verifies that `CONSUMER_FUTURE_KIND` is a non-empty, recognisable string
//! usable in tracing spans and metric labels.

use swe_edge_ingress_message_broker_transport::CONSUMER_FUTURE_KIND;

/// @covers: api/types/consumer_future.rs::CONSUMER_FUTURE_KIND — is non-empty.
#[test]
fn test_consumer_future_kind_const_is_nonempty_happy() {
    assert!(
        !CONSUMER_FUTURE_KIND.is_empty(),
        "CONSUMER_FUTURE_KIND must not be empty"
    );
}

/// @covers: api/types/consumer_future.rs::CONSUMER_FUTURE_KIND — contains the crate domain.
#[test]
fn test_consumer_future_kind_const_contains_consumer_segment_error() {
    // A wrong value would silently produce metrics under an unrecognised label;
    // assert the constant contains a recognisable domain token.
    assert!(
        CONSUMER_FUTURE_KIND.contains("consumer"),
        "CONSUMER_FUTURE_KIND must contain 'consumer'; got: {CONSUMER_FUTURE_KIND}"
    );
}

/// @covers: api/types/consumer_future.rs::CONSUMER_FUTURE_KIND — usable in a format string.
#[test]
fn test_consumer_future_kind_const_embeds_in_span_name_edge() {
    let span_name = format!("{CONSUMER_FUTURE_KIND}.subscribe");
    assert!(span_name.contains("subscribe"), "span must include method name");
}
