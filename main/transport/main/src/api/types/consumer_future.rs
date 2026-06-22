//! Consumer future identifier constant.
//!
//! `SubscribeFuture` and `HealthCheckFuture` are each defined in their own
//! files (`subscribe_future.rs`, `health_check_future.rs`). This file provides
//! the shared future kind identifier used in log and trace annotations.

/// Identifies the consumer-subscribe future kind in tracing spans and metrics.
///
/// Used as a discriminator when logging or recording metrics for async
/// consumer operations, to distinguish from non-consumer futures.
pub const CONSUMER_FUTURE_KIND: &str = "ingress-message-broker-consumer";
