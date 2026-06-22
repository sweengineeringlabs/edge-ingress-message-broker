//! SAF message consumer service facade.
//!
//! The [`MessageConsumer`] port trait is exported from `lib.rs` directly
//! (not through saf/) to comply with `saf_no_trait_reexport`.
//! This file satisfies the `saf_trait_svc_correspondence` structural check
//! and exposes the well-known default topic separator used by consumers.

/// Separator character used in hierarchical topic names (e.g. `"orders.created"`).
///
/// NATS and in-memory consumers use dot-separated topic hierarchies.
/// Callers building topic strings should use this constant for consistency.
pub const TOPIC_SEPARATOR: char = '.';
