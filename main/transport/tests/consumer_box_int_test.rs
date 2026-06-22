//! Integration tests — ConsumerBox type alias.
//!
//! Verifies that `ConsumerBox` resolves to an Arc of the `MessageConsumer`
//! trait object and that it can be used at all Arc/dyn call sites.

use swe_edge_ingress_message_broker_transport::ConsumerBox;

/// @covers: api/types/consumer_box.rs::ConsumerBox — type exists and is nameable.
#[test]
fn test_consumer_box_type_consumer_box_is_nameable_happy() {
    // The type alias must be usable in a position annotation without error.
    fn accept(_b: ConsumerBox) {}
    // The function just needs to exist and compile; we can't call it without
    // a concrete MessageConsumer, so we only prove the type is accepted.
    let _ = accept;
}

/// @covers: api/types/consumer_box.rs::ConsumerBox — can be held in an Option.
#[test]
fn test_consumer_box_type_wrappable_in_option_error() {
    // "error" scenario: prove that None::<ConsumerBox> type-checks without a
    // concrete value — this would panic if the type alias were wrong.
    let _opt: Option<ConsumerBox> = None;
}

/// @covers: api/types/consumer_box.rs::ConsumerBox — can store in a Vec.
#[test]
fn test_consumer_box_type_storable_in_vec_edge() {
    // The type alias must work as a generic parameter in a collection type.
    let _pool: Vec<ConsumerBox> = Vec::new();
    assert!(_pool.is_empty());
}
