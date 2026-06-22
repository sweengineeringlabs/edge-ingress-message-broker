//! Integration tests — Validator trait contract.
//!
//! Covers `validate`, `as_consumer_config`, `as_nats_config`,
//! `as_transport_svc`, and `to_config_builder` via concrete implementations.

use swe_edge_ingress_message_broker_transport::{
    MessageConsumerConfig, NatsConsumerConfig, TransportSvc, Validator,
};

// ---------------------------------------------------------------------------
// validate — happy / error / edge
// ---------------------------------------------------------------------------

/// @covers: Validator::validate — non-zero default capacity is accepted.
#[test]
fn test_validate_default_consumer_config_passes_validation_happy() {
    assert!(
        TransportSvc::validate(&MessageConsumerConfig::default()).is_ok(),
        "default capacity (1024) must pass validation"
    );
}

/// @covers: Validator::validate — zero capacity is rejected with an error string.
#[test]
fn test_validate_zero_capacity_config_returns_err_error() {
    let cfg = MessageConsumerConfig { capacity: 0 };
    let result = TransportSvc::validate(&cfg);
    assert!(result.is_err(), "capacity == 0 must fail validation");
    let msg = result.expect_err("already checked is_err");
    assert!(
        msg.contains("capacity"),
        "error message must mention 'capacity'; got: {msg}"
    );
}

/// @covers: Validator::validate — custom Validator impl is dispatched polymorphically.
#[test]
fn test_validate_custom_impl_is_dispatched_correctly_edge() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    struct AlwaysErr;
    impl Validator for AlwaysErr {
        fn validate(&self) -> Result<(), String> {
            Err("always-err".into())
        }
    }

    assert!(TransportSvc::validate(&AlwaysOk).is_ok());
    assert!(TransportSvc::validate(&AlwaysErr).is_err());
}

// ---------------------------------------------------------------------------
// as_consumer_config — happy / error / edge
// ---------------------------------------------------------------------------

/// @covers: Validator::as_consumer_config — MessageConsumerConfig returns Some(self).
#[test]
fn test_as_consumer_config_message_consumer_config_returns_some_happy() {
    let cfg = MessageConsumerConfig::default();
    assert!(
        cfg.as_consumer_config().is_some(),
        "MessageConsumerConfig must return Some(&self)"
    );
}

/// @covers: Validator::as_consumer_config — unrelated impl returns None.
#[test]
fn test_as_consumer_config_unrelated_impl_returns_none_error() {
    struct Irrelevant;
    impl Validator for Irrelevant {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(
        Irrelevant.as_consumer_config().is_none(),
        "unrelated Validator impl must return None from as_consumer_config"
    );
}

/// @covers: Validator::as_consumer_config — returned reference preserves capacity.
#[test]
fn test_as_consumer_config_returned_ref_has_correct_capacity_edge() {
    let cfg = MessageConsumerConfig { capacity: 512 };
    let referred = cfg.as_consumer_config().expect("must be Some");
    assert_eq!(referred.capacity, 512);
}

// ---------------------------------------------------------------------------
// as_nats_config — happy / error / edge
// ---------------------------------------------------------------------------

/// @covers: Validator::as_nats_config — NatsConsumerConfig returns Some(self).
#[test]
fn test_as_nats_config_nats_consumer_config_returns_some_happy() {
    let cfg = NatsConsumerConfig {
        url: "nats://localhost:4222".into(),
    };
    assert!(cfg.as_nats_config().is_some());
}

/// @covers: Validator::as_nats_config — unrelated impl returns None.
#[test]
fn test_as_nats_config_unrelated_impl_returns_none_error() {
    struct Irrelevant;
    impl Validator for Irrelevant {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(Irrelevant.as_nats_config().is_none());
}

/// @covers: Validator::as_nats_config — returned reference preserves URL.
#[test]
fn test_as_nats_config_returned_ref_has_correct_url_edge() {
    let cfg = NatsConsumerConfig {
        url: "nats://prod.example.com:4222".into(),
    };
    let referred = cfg.as_nats_config().expect("must be Some");
    assert_eq!(referred.url, "nats://prod.example.com:4222");
}

// ---------------------------------------------------------------------------
// as_transport_svc — happy / error / edge
// ---------------------------------------------------------------------------

/// @covers: Validator::as_transport_svc — default impl returns None.
#[test]
fn test_as_transport_svc_default_impl_returns_none_happy() {
    let cfg = MessageConsumerConfig::default();
    // MessageConsumerConfig does not override as_transport_svc,
    // so the default must return None.
    assert!(cfg.as_transport_svc().is_none());
}

/// @covers: Validator::as_transport_svc — custom impl can return Some.
#[test]
fn test_as_transport_svc_custom_impl_returns_some_error() {
    use swe_edge_ingress_message_broker_transport::TransportSvc as Svc;

    struct Carrier;
    impl Validator for Carrier {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
        fn as_transport_svc(&self) -> Option<Svc> {
            Some(Svc)
        }
    }

    assert!(Carrier.as_transport_svc().is_some());
}

/// @covers: Validator::as_transport_svc — dyn Validator dispatch is correct.
#[test]
fn test_as_transport_svc_via_dyn_trait_returns_none_edge() {
    let v: &dyn Validator = &MessageConsumerConfig::default();
    assert!(v.as_transport_svc().is_none());
}

// ---------------------------------------------------------------------------
// to_config_builder — happy / error / edge
// ---------------------------------------------------------------------------

/// @covers: Validator::to_config_builder — MessageConsumerConfig returns Some builder.
#[test]
fn test_to_config_builder_message_consumer_config_returns_some_happy() {
    let cfg = MessageConsumerConfig { capacity: 256 };
    let builder = cfg.to_config_builder();
    assert!(builder.is_some(), "MessageConsumerConfig must return Some(builder)");
}

/// @covers: Validator::to_config_builder — returned builder produces the original capacity.
#[test]
fn test_to_config_builder_builder_preserves_capacity_error() {
    // "error" scenario: a wrong capacity would cause silent misconfiguration.
    let cfg = MessageConsumerConfig { capacity: 256 };
    let built = cfg
        .to_config_builder()
        .expect("must be Some")
        .build();
    assert_eq!(
        built.capacity, 256,
        "builder built from cfg must preserve capacity"
    );
}

/// @covers: Validator::to_config_builder — unrelated impl returns None.
#[test]
fn test_to_config_builder_unrelated_impl_returns_none_edge() {
    struct Opaque;
    impl Validator for Opaque {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(
        Opaque.to_config_builder().is_none(),
        "types that do not override to_config_builder must return None"
    );
}
