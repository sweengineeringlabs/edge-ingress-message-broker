//! Integration tests — Validator trait via SAF.

use swe_edge_ingress_message_broker_transport::{MessageConsumerConfig, TransportSvc, Validator};

/// @covers: TransportSvc::validate — delegates to the Validator impl; non-zero default capacity passes.
#[test]
fn test_validate_default_consumer_config_capacity_nonzero_returns_ok() {
    assert!(TransportSvc::validate(&MessageConsumerConfig::default()).is_ok());
}

/// @covers: TransportSvc::validate — propagates Err from the Validator impl.
#[test]
fn test_validate_returns_err_for_zero_capacity() {
    struct Zero;
    impl Validator for Zero {
        fn validate(&self) -> Result<(), String> {
            Err("zero".into())
        }
    }
    assert!(TransportSvc::validate(&Zero).is_err());
}
