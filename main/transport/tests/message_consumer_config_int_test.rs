//! Integration tests — MessageConsumerConfig.

use swe_edge_ingress_message_broker_transport::{MessageConsumerConfig, TransportSvc, Validator};

/// @covers: api/types/message_consumer_config.rs — default capacity is 1024
#[test]
fn test_message_consumer_config_default_capacity_is_1024() {
    assert_eq!(MessageConsumerConfig::default().capacity, 1024);
}

/// @covers: api/types/message_consumer_config.rs — validate passes for non-zero capacity
#[test]
fn test_message_consumer_config_validate_passes_for_non_zero_capacity() {
    assert!(TransportSvc::validate(&MessageConsumerConfig::default()).is_ok());
}

/// @covers: api/types/message_consumer_config.rs — validate fails for zero capacity
#[test]
fn test_message_consumer_config_validate_fails_for_zero_capacity() {
    let cfg = MessageConsumerConfig { capacity: 0 };
    assert!(cfg.validate().is_err());
}
