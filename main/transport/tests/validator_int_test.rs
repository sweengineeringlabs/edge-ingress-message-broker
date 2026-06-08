//! Integration tests — Validator trait via SAF.

use swe_edge_ingress_message_broker_transport::{validate, MessageConsumerConfig, TransportSvc, Validator};

/// @covers: validate — delegates to the Validator impl; non-zero default capacity passes.
#[test]
fn test_validate_default_consumer_config_capacity_nonzero_returns_ok() {
    assert!(validate(&MessageConsumerConfig::default()).is_ok());
}

/// @covers: validate — propagates Err from the Validator impl.
#[test]
fn test_validate_returns_err_for_zero_capacity() {
    struct Zero;
    impl Validator for Zero {
        fn validate(&self) -> Result<(), String> {
            Err("zero".into())
        }
    }
    assert!(validate(&Zero).is_err());
}

/// @covers: TransportSvc::validate — delegates to the Validator impl.
#[test]
fn test_transport_svc_validate_delegates_to_impl() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        fn validate(&self) -> Result<(), String> { Ok(()) }
    }
    assert!(TransportSvc::validate(&AlwaysOk).is_ok());
}
