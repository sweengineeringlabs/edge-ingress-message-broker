//! SAF NATS message consumer service facade.
//!
//! The [`NatsMessageConsumer`] marker trait is exported from `lib.rs` directly
//! (not through saf/) to comply with `saf_no_trait_reexport`.
//! This file satisfies the `saf_trait_svc_correspondence` structural check
//! and exposes NATS-specific constants used by transport configuration.

/// Default NATS server URL used when no explicit URL is configured.
///
/// Points to a localhost NATS server on the default port (4222).
pub const DEFAULT_NATS_URL: &str = "nats://localhost:4222";
