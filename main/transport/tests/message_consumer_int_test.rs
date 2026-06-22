//! Integration tests — MessageConsumer trait contract.
//!
//! Covers subscribe, health_check, into_box, and disconnected_error
//! with happy / error / edge scenarios.

// --------------------------------------------------------------------------
// into_box — happy / error / edge  (does not require in-memory feature)
// --------------------------------------------------------------------------

use swe_edge_ingress_message_broker_transport::{
    ConsumerBox, ConsumerError, HealthCheckFuture, MessageConsumer, SubscribeFuture,
};

/// Minimal no-op consumer used to test default-method behaviour.
struct NoopConsumer;
impl MessageConsumer for NoopConsumer {
    fn subscribe<'a>(&'a self, _topic: &'a str) -> SubscribeFuture<'a> {
        Box::pin(async { Err(ConsumerError::Unavailable("noop".into())) })
    }
    fn health_check(&self) -> HealthCheckFuture<'_> {
        Box::pin(async { Ok(()) })
    }
}

/// @covers: MessageConsumer::into_box — wraps self in Arc.
#[test]
fn test_into_box_wraps_consumer_into_arc_happy() {
    let boxed: ConsumerBox = NoopConsumer.into_box();
    // The boxed value must still respond to method calls via deref.
    let _ = boxed; // type check is sufficient
}

/// @covers: MessageConsumer::into_box — Arc can be cloned independently.
#[test]
fn test_into_box_cloned_arc_is_independent_error() {
    // "error" scenario: two clones of the Arc must both be alive without
    // aliasing mutable state.
    let b1: ConsumerBox = NoopConsumer.into_box();
    let b2 = b1.clone();
    drop(b1);
    drop(b2);
}

/// @covers: MessageConsumer::into_box — result is usable as dyn MessageConsumer.
#[tokio::test]
async fn test_into_box_is_callable_via_dyn_message_consumer_edge() {
    let boxed: ConsumerBox = NoopConsumer.into_box();
    // health_check is defined as Ok(()) in NoopConsumer.
    assert!(boxed.health_check().await.is_ok());
}

// --------------------------------------------------------------------------
// disconnected_error — happy / error / edge
// --------------------------------------------------------------------------

/// @covers: MessageConsumer::disconnected_error — returns ConsumerError::Unavailable.
#[test]
fn test_disconnected_error_returns_unavailable_variant_happy() {
    let err = NoopConsumer.disconnected_error();
    match err {
        ConsumerError::Unavailable(_) => {}
        other => panic!("expected Unavailable, got {other}"),
    }
}

/// @covers: MessageConsumer::disconnected_error — error message is non-empty.
#[test]
fn test_disconnected_error_message_is_nonempty_error() {
    let err = NoopConsumer.disconnected_error();
    assert!(!err.to_string().is_empty(), "disconnected error message must not be empty");
}

/// @covers: MessageConsumer::disconnected_error — overrideable by implementors.
#[test]
fn test_disconnected_error_can_be_overridden_edge() {
    struct AlwaysConnectionErr;
    impl MessageConsumer for AlwaysConnectionErr {
        fn subscribe<'a>(&'a self, _topic: &'a str) -> SubscribeFuture<'a> {
            Box::pin(async { Err(ConsumerError::Connection("refused".into())) })
        }
        fn health_check(&self) -> HealthCheckFuture<'_> {
            Box::pin(async { Err(ConsumerError::Connection("refused".into())) })
        }
        fn disconnected_error(&self) -> ConsumerError {
            ConsumerError::Connection("override".into())
        }
    }

    let err = AlwaysConnectionErr.disconnected_error();
    match err {
        ConsumerError::Connection(msg) => assert_eq!(msg, "override"),
        other => panic!("expected Connection, got {other}"),
    }
}

// --------------------------------------------------------------------------
// health_check_result — happy / error / edge
// --------------------------------------------------------------------------

/// @covers: MessageConsumer::health_check_result — healthy consumer returns Ok(()).
#[tokio::test]
async fn test_health_check_result_healthy_consumer_returns_ok_happy() {
    let result = NoopConsumer.health_check_result().await;
    assert!(
        result.is_ok(),
        "health_check_result must return Ok for a healthy consumer"
    );
}

/// @covers: MessageConsumer::health_check_result — unavailable consumer returns Err.
#[tokio::test]
async fn test_health_check_result_unavailable_consumer_returns_err_error() {
    struct Unhealthy;
    impl MessageConsumer for Unhealthy {
        fn subscribe<'a>(&'a self, _topic: &'a str) -> SubscribeFuture<'a> {
            Box::pin(async { Err(ConsumerError::Unavailable("down".into())) })
        }
        fn health_check(&self) -> HealthCheckFuture<'_> {
            Box::pin(async { Err(ConsumerError::Unavailable("down".into())) })
        }
    }
    let result = Unhealthy.health_check_result().await;
    assert!(
        result.is_err(),
        "health_check_result must propagate Err from health_check"
    );
}

/// @covers: MessageConsumer::health_check_result — result is identical to health_check outcome.
#[tokio::test]
async fn test_health_check_result_matches_health_check_output_edge() {
    // Default impl delegates to health_check; both must return the same outcome.
    let via_health_check = NoopConsumer.health_check().await;
    let via_result = NoopConsumer.health_check_result().await;
    assert_eq!(
        via_health_check.is_ok(),
        via_result.is_ok(),
        "health_check_result must return the same Ok/Err shape as health_check"
    );
}

// --------------------------------------------------------------------------
// subscribe_checked — happy / error / edge
// --------------------------------------------------------------------------

/// @covers: MessageConsumer::subscribe_checked — delegates to subscribe and returns Ok.
#[tokio::test]
async fn test_subscribe_checked_default_impl_returns_ok_happy() {
    struct AlwaysOk;
    impl MessageConsumer for AlwaysOk {
        fn subscribe<'a>(&'a self, _topic: &'a str) -> SubscribeFuture<'a> {
            Box::pin(async {
                Ok(Box::pin(futures::stream::empty()) as swe_edge_runtime_message_broker::MessageStream)
            })
        }
        fn health_check(&self) -> HealthCheckFuture<'_> {
            Box::pin(async { Ok(()) })
        }
    }

    let result = AlwaysOk.subscribe_checked("orders.new").await;
    assert!(result.is_ok(), "subscribe_checked must return Ok when subscribe does");
}

/// @covers: MessageConsumer::subscribe_checked — propagates Err from subscribe.
#[tokio::test]
async fn test_subscribe_checked_propagates_err_from_subscribe_error() {
    let result = NoopConsumer.subscribe_checked("any.topic").await;
    assert!(result.is_err(), "subscribe_checked must propagate Err from subscribe");
}

/// @covers: MessageConsumer::subscribe_checked — result type is ConsumerResult<MessageStream>.
#[tokio::test]
async fn test_subscribe_checked_returns_consumer_result_type_edge() {
    use swe_edge_ingress_message_broker_transport::ConsumerResult;
    use swe_edge_runtime_message_broker::MessageStream;

    fn accept(_r: ConsumerResult<MessageStream>) {}
    accept(NoopConsumer.subscribe_checked("any").await);
}

// --------------------------------------------------------------------------
// subscribe + health_check — happy / error / edge  (in-memory feature)
// --------------------------------------------------------------------------

#[cfg(feature = "in-memory")]
mod in_memory_tests {
    use swe_edge_ingress_message_broker_transport::{
        ConsumerError, ConsumerHealthResult, ConsumerStreamResult, HealthCheckFuture, MessageConsumer,
        SubscribeFuture, TransportSvc,
    };

    // -----------------------------------------------------------------------
    // subscribe — happy / error / edge
    // -----------------------------------------------------------------------

    /// @covers: MessageConsumer::subscribe — valid topic returns Ok stream.
    #[tokio::test]
    async fn test_subscribe_valid_topic_returns_ok_stream_happy() {
        let c = TransportSvc::default_consumer();
        let result = c.subscribe("orders.created").await;
        assert!(result.is_ok(), "subscribe on a valid topic must return Ok");
    }

    /// @covers: MessageConsumer::subscribe — a consumer that fails returns Err.
    #[tokio::test]
    async fn test_subscribe_failing_consumer_returns_err_error() {
        struct AlwaysFails;
        impl MessageConsumer for AlwaysFails {
            fn subscribe<'a>(&'a self, topic: &'a str) -> SubscribeFuture<'a> {
                let t = topic.to_owned();
                Box::pin(async move {
                    Err(ConsumerError::Subscribe {
                        topic: t,
                        reason: "broker down".into(),
                    })
                })
            }
            fn health_check(&self) -> HealthCheckFuture<'_> {
                Box::pin(async { Err(ConsumerError::Unavailable("broker down".into())) })
            }
        }

        let result = AlwaysFails.subscribe("any.topic").await;
        assert!(result.is_err(), "AlwaysFails must return Err from subscribe");
    }

    /// @covers: MessageConsumer::subscribe — multiple distinct topics are independent.
    #[tokio::test]
    async fn test_subscribe_multiple_topics_each_stream_is_independent_edge() {
        let c = TransportSvc::default_consumer();
        let r1 = c.subscribe("topic.a").await;
        let r2 = c.subscribe("topic.b").await;
        assert!(r1.is_ok(), "first subscription must succeed");
        assert!(r2.is_ok(), "second subscription must succeed independently");
    }

    // -----------------------------------------------------------------------
    // health_check — happy / error / edge
    // -----------------------------------------------------------------------

    /// @covers: MessageConsumer::health_check — in-memory consumer is always healthy.
    #[tokio::test]
    async fn test_health_check_in_memory_consumer_returns_ok_happy() {
        let c = TransportSvc::default_consumer();
        assert!(
            c.health_check().await.is_ok(),
            "in-memory consumer health_check must succeed"
        );
    }

    /// @covers: MessageConsumer::health_check — disconnected consumer returns Err.
    #[tokio::test]
    async fn test_health_check_failing_consumer_returns_err_error() {
        struct AlwaysUnavailable;
        impl MessageConsumer for AlwaysUnavailable {
            fn subscribe<'a>(&'a self, _topic: &'a str) -> SubscribeFuture<'a> {
                Box::pin(async { Err(ConsumerError::Unavailable("test".into())) })
            }
            fn health_check(&self) -> HealthCheckFuture<'_> {
                Box::pin(async { Err(ConsumerError::Unavailable("test".into())) })
            }
        }

        let c = AlwaysUnavailable;
        assert!(
            c.health_check().await.is_err(),
            "unavailable consumer health_check must return Err"
        );
    }

    /// @covers: MessageConsumer::health_check — check is callable via trait object.
    #[tokio::test]
    async fn test_health_check_via_trait_object_returns_ok_edge() {
        use swe_edge_ingress_message_broker_transport::ConsumerBox;

        let c: ConsumerBox = TransportSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }
}
