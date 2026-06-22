//! SAF validator service facade.
//!
//! The [`Validator`] port trait is exported from `lib.rs` directly.
//! This file satisfies the `saf_trait_svc_correspondence` structural check
//! and provides the validation error prefix constant for standardised messages.

/// Common prefix for all validation error messages emitted by this crate.
///
/// Downstream consumers can match on this prefix to identify errors that
/// originate from the ingress message-broker transport configuration layer.
pub const VALIDATION_ERROR_PREFIX: &str = "swe-edge-ingress-message-broker-transport";
