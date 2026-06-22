//! Integration tests — HealthCheckFuture type alias.
//!
//! Verifies that `HealthCheckFuture<'_>` resolves correctly and can be used
//! as a function return type.

use swe_edge_ingress_message_broker_transport::{ConsumerError, HealthCheckFuture};

/// @covers: api/types/health_check_future.rs::HealthCheckFuture — Ok(()) resolves correctly.
#[tokio::test]
async fn test_health_check_future_type_ok_resolves_to_unit_happy() {
    fn healthy_future() -> HealthCheckFuture<'static> {
        Box::pin(async { Ok(()) })
    }

    assert!(healthy_future().await.is_ok());
}

/// @covers: api/types/health_check_future.rs::HealthCheckFuture — Err resolves to ConsumerError.
#[tokio::test]
async fn test_health_check_future_type_err_resolves_to_consumer_error_error() {
    fn sick_future() -> HealthCheckFuture<'static> {
        Box::pin(async { Err(ConsumerError::Unavailable("down".into())) })
    }

    match sick_future().await {
        Err(ConsumerError::Unavailable(msg)) => assert_eq!(msg, "down"),
        other => panic!("unexpected result: {other:?}"),
    }
}

/// @covers: api/types/health_check_future.rs::HealthCheckFuture — storable in Option.
#[test]
fn test_health_check_future_type_storable_in_option_edge() {
    let _: Option<HealthCheckFuture<'_>> = None;
}
