//! SAF types facade — type surface for the ingress message consumer.
//!
//! This file is the corresponding `_svc.rs` for value types declared in api/types/.
//! Concrete types are exported from `lib.rs` directly to avoid SEA boundary violations.
//! This file exposes the default capacity constant that consumers use to seed builders.

/// Default channel capacity for the ingress message consumer.
///
/// Consumers that do not load configuration from TOML fall back to this value.
/// Equivalent to `MessageConsumerConfig::default().capacity`.
pub const DEFAULT_CONSUMER_CAPACITY: usize = 1024;
