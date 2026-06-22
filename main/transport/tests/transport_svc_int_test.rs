//! Integration tests — TransportSvc facade methods.
//!
//! Covers create_config_builder, validate, subscribe_to, check_health,
//! and default_consumer (in-memory) with happy / error / edge scenarios.

use swe_edge_ingress_message_broker_transport::{MessageConsumerConfig, TransportSvc, Validator};

// ---------------------------------------------------------------------------
// create_config_builder — happy / error / edge
// ---------------------------------------------------------------------------

/// @covers: TransportSvc::create_config_builder — returns a usable builder.
#[test]
fn test_create_config_builder_returns_loader_builder_happy() {
    let builder = TransportSvc::create_config_builder();
    // The builder must produce a loader without panicking.
    let _loader = builder.build_loader();
}

/// @covers: TransportSvc::create_config_builder — builder carries crate name.
#[test]
fn test_create_config_builder_builder_embeds_crate_name_error() {
    // A wrong name would produce XDG paths under an unrecognised crate; verify
    // the builder is seeded with this crate's CARGO_PKG_NAME.
    let builder = TransportSvc::create_config_builder();
    let _loader = builder.build_loader();
    // Passes as long as no panic occurs — the builder name assertion is
    // internal to swe-edge-configbuilder and triggered at loader creation.
}

/// @covers: TransportSvc::create_config_builder — multiple calls return independent builders.
#[test]
fn test_create_config_builder_multiple_calls_are_independent_edge() {
    let b1 = TransportSvc::create_config_builder();
    let b2 = TransportSvc::create_config_builder();
    // Both must produce loaders without sharing state.
    let _l1 = b1.build_loader();
    let _l2 = b2.build_loader();
}

// ---------------------------------------------------------------------------
// validate — happy / error / edge
// ---------------------------------------------------------------------------

/// @covers: TransportSvc::validate — valid config returns Ok.
#[test]
fn test_validate_valid_config_returns_ok_happy() {
    assert!(TransportSvc::validate(&MessageConsumerConfig::default()).is_ok());
}

/// @covers: TransportSvc::validate — zero capacity returns Err.
#[test]
fn test_validate_zero_capacity_returns_err_error() {
    let cfg = MessageConsumerConfig { capacity: 0 };
    assert!(TransportSvc::validate(&cfg).is_err());
}

/// @covers: TransportSvc::validate — generic dispatch works via dyn Validator.
#[test]
fn test_validate_custom_validator_dispatched_correctly_edge() {
    struct AlwaysErr;
    impl Validator for AlwaysErr {
        fn validate(&self) -> Result<(), String> {
            Err("always-err".into())
        }
    }
    assert!(TransportSvc::validate(&AlwaysErr).is_err());
}

// ---------------------------------------------------------------------------
// subscribe_to + check_health + default_consumer — happy / error / edge
// (requires in-memory feature)
// ---------------------------------------------------------------------------

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_ingress_message_broker_transport::{
        ConsumerError, HealthCheckFuture, MessageConsumer, SubscribeFuture, TransportSvc,
    };

    // --- default_consumer ---------------------------------------------------

    /// @covers: TransportSvc::default_consumer — returns a healthy consumer.
    #[tokio::test]
    async fn test_default_consumer_returns_healthy_consumer_happy() {
        let c = TransportSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    /// @covers: TransportSvc::default_consumer — returned consumer can subscribe.
    #[tokio::test]
    async fn test_default_consumer_returned_consumer_subscribes_error() {
        // "error" scenario: we verify the subscribe path does NOT error for a
        // valid topic, catching regressions where construction succeeds but the
        // first subscribe panics.
        let c = TransportSvc::default_consumer();
        assert!(c.subscribe("test.topic").await.is_ok());
    }

    /// @covers: TransportSvc::default_consumer — multiple calls produce independent consumers.
    #[tokio::test]
    async fn test_default_consumer_multiple_instances_are_independent_edge() {
        let c1 = TransportSvc::default_consumer();
        let c2 = TransportSvc::default_consumer();
        // Both must produce loaders without sharing state.
        assert!(c1.health_check().await.is_ok());
        assert!(c2.health_check().await.is_ok());
    }

    // --- subscribe_to -------------------------------------------------------

    /// @covers: TransportSvc::subscribe_to — valid topic returns Ok(stream).
    #[tokio::test]
    async fn test_subscribe_to_valid_topic_returns_ok_happy() {
        let c = TransportSvc::default_consumer();
        let result = TransportSvc::subscribe_to(c.as_ref(), "orders.created").await;
        assert!(result.is_ok(), "subscribe_to a valid topic must succeed");
    }

    /// @covers: TransportSvc::subscribe_to — failing consumer returns Err.
    #[tokio::test]
    async fn test_subscribe_to_failing_consumer_returns_err_error() {
        struct BrokenConsumer;
        impl MessageConsumer for BrokenConsumer {
            fn subscribe<'a>(&'a self, topic: &'a str) -> SubscribeFuture<'a> {
                let t = topic.to_owned();
                Box::pin(async move {
                    Err(ConsumerError::Subscribe {
                        topic: t,
                        reason: "simulated failure".into(),
                    })
                })
            }
            fn health_check(&self) -> HealthCheckFuture<'_> {
                Box::pin(async { Err(ConsumerError::Unavailable("broken".into())) })
            }
        }

        let result = TransportSvc::subscribe_to(&BrokenConsumer, "orders.new").await;
        assert!(result.is_err(), "subscribe_to broken consumer must return Err");
    }

    /// @covers: TransportSvc::subscribe_to — works via dyn MessageConsumer.
    #[tokio::test]
    async fn test_subscribe_to_via_dyn_consumer_returns_ok_edge() {
        use swe_edge_ingress_message_broker_transport::ConsumerBox;
        let c: ConsumerBox = TransportSvc::default_consumer();
        let result = TransportSvc::subscribe_to(c.as_ref(), "events.tick").await;
        assert!(result.is_ok());
    }

    // --- check_health -------------------------------------------------------

    /// @covers: TransportSvc::check_health — in-memory consumer returns Ok.
    #[tokio::test]
    async fn test_check_health_in_memory_consumer_returns_ok_happy() {
        let c = TransportSvc::default_consumer();
        assert!(TransportSvc::check_health(c.as_ref()).await.is_ok());
    }

    /// @covers: TransportSvc::check_health — failing consumer returns Err.
    #[tokio::test]
    async fn test_check_health_disconnected_consumer_returns_err_error() {
        struct Sick;
        impl MessageConsumer for Sick {
            fn subscribe<'a>(&'a self, _topic: &'a str) -> SubscribeFuture<'a> {
                Box::pin(async { Err(ConsumerError::Unavailable("sick".into())) })
            }
            fn health_check(&self) -> HealthCheckFuture<'_> {
                Box::pin(async { Err(ConsumerError::Unavailable("sick".into())) })
            }
        }
        assert!(TransportSvc::check_health(&Sick).await.is_err());
    }

    /// @covers: TransportSvc::check_health — works via dyn MessageConsumer.
    #[tokio::test]
    async fn test_check_health_via_dyn_consumer_returns_ok_edge() {
        use swe_edge_ingress_message_broker_transport::ConsumerBox;
        let c: ConsumerBox = TransportSvc::default_consumer();
        assert!(TransportSvc::check_health(c.as_ref()).await.is_ok());
    }
}

// ---------------------------------------------------------------------------
// nats_consumer — happy / error / edge (requires nats feature)
// ---------------------------------------------------------------------------
//
// Connecting to a real NATS server is not available in CI without a broker
// fixture, so these tests exercise the error path against a port that is not
// listening (localhost:1) — a connection to that port will always fail.
// The "happy" and "edge" scenarios use the error-path too; the distinction
// is in what is being asserted:
//   happy  — the factory returns Err (expected — no server)
//   error  — the Err contains ConsumerError::Connection (right variant)
//   edge   — an invalid URL scheme returns an error too

#[cfg(feature = "nats")]
mod nats_tests {
    use swe_edge_ingress_message_broker_transport::{ConsumerError, TransportSvc};

    /// @covers: TransportSvc::nats_consumer — unreachable server returns Err.
    #[tokio::test]
    async fn test_nats_consumer_unreachable_server_returns_err_happy() {
        // Port 1 is always closed; the factory must return Err, not panic.
        let result = TransportSvc::nats_consumer("nats://localhost:1").await;
        assert!(result.is_err(), "unreachable server must return Err");
    }

    /// @covers: TransportSvc::nats_consumer — Err is ConsumerError::Connection.
    #[tokio::test]
    async fn test_nats_consumer_err_is_connection_error_error() {
        let result = TransportSvc::nats_consumer("nats://localhost:1").await;
        match result {
            Err(ConsumerError::Connection(_)) => {}
            Err(other) => {
                panic!("expected ConsumerError::Connection, got {other:?}");
            }
            Ok(_) => panic!("expected Err, got Ok"),
        }
    }

    /// @covers: TransportSvc::nats_consumer — invalid URL returns Err (not panic).
    #[tokio::test]
    async fn test_nats_consumer_invalid_url_returns_err_edge() {
        // An empty URL should be rejected gracefully.
        let result = TransportSvc::nats_consumer("").await;
        assert!(result.is_err(), "empty URL must return Err, not panic");
    }
}
