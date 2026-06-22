//! Integration tests — SAF types service default capacity constant.
//!
//! Verifies that `DEFAULT_CONSUMER_CAPACITY` agrees with the default
//! `MessageConsumerConfig` capacity so the two sources of truth stay in sync.

use swe_edge_ingress_message_broker_transport::{
    MessageConsumerConfig, DEFAULT_CONSUMER_CAPACITY,
};

/// @covers: saf/types_svc.rs::DEFAULT_CONSUMER_CAPACITY — agrees with MessageConsumerConfig::default().
#[test]
fn test_default_consumer_capacity_const_matches_config_default_happy() {
    assert_eq!(
        DEFAULT_CONSUMER_CAPACITY,
        MessageConsumerConfig::default().capacity,
        "DEFAULT_CONSUMER_CAPACITY must equal MessageConsumerConfig::default().capacity"
    );
}

/// @covers: saf/types_svc.rs::DEFAULT_CONSUMER_CAPACITY — is a positive capacity.
#[test]
fn test_default_consumer_capacity_const_is_nonzero_error() {
    // A zero capacity would deadlock the broadcast channel; verify the
    // constant is strictly positive so validation never rejects the default.
    assert!(
        DEFAULT_CONSUMER_CAPACITY > 0,
        "DEFAULT_CONSUMER_CAPACITY must be > 0; got {DEFAULT_CONSUMER_CAPACITY}"
    );
}

/// @covers: saf/types_svc.rs::DEFAULT_CONSUMER_CAPACITY — seeds ApplicationConfigBuilder correctly.
#[test]
fn test_default_consumer_capacity_const_seeds_builder_edge() {
    use swe_edge_ingress_message_broker_transport::ApplicationConfigBuilder;

    let cfg = ApplicationConfigBuilder::default()
        .with_capacity(DEFAULT_CONSUMER_CAPACITY)
        .build();
    assert_eq!(cfg.capacity, DEFAULT_CONSUMER_CAPACITY);
}
