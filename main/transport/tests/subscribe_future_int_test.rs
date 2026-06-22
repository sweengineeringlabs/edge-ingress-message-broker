//! Integration tests — SubscribeFuture type alias.
//!
//! Verifies that `SubscribeFuture<'a>` resolves correctly and can be used
//! as a function return type.

use swe_edge_ingress_message_broker_transport::{ConsumerError, SubscribeFuture};

/// @covers: api/types/subscribe_future.rs::SubscribeFuture — type is nameable as fn return.
#[tokio::test]
async fn test_subscribe_future_type_usable_as_fn_return_happy() {
    fn make_err_future<'a>(topic: &'a str) -> SubscribeFuture<'a> {
        let t = topic.to_owned();
        Box::pin(async move {
            Err(ConsumerError::Subscribe {
                topic: t,
                reason: "simulated".into(),
            })
        })
    }

    let result = make_err_future("test.topic").await;
    assert!(result.is_err(), "returned future must resolve to Err");
}

/// @covers: api/types/subscribe_future.rs::SubscribeFuture — error case resolves correctly.
#[tokio::test]
async fn test_subscribe_future_type_err_resolves_with_consumer_error_error() {
    fn failing_future<'a>(_topic: &'a str) -> SubscribeFuture<'a> {
        Box::pin(async { Err(ConsumerError::Unavailable("no broker".into())) })
    }

    let result = failing_future("x").await;
    match result {
        Err(ConsumerError::Unavailable(msg)) => assert_eq!(msg, "no broker"),
        Err(other) => panic!("expected Unavailable, got: {other}"),
        Ok(_) => panic!("expected Err, got Ok"),
    }
}

/// @covers: api/types/subscribe_future.rs::SubscribeFuture — storable in a type alias position.
#[test]
fn test_subscribe_future_type_usable_in_option_edge() {
    // Prove the type alias can be named in generic context without a value.
    let _: Option<SubscribeFuture<'_>> = None;
}
