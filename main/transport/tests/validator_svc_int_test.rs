//! Integration tests — SAF validator service constants.
//!
//! Verifies that `VALIDATION_ERROR_PREFIX` is well-formed and matches the
//! crate name so downstream consumers can reliably filter errors by source.

use swe_edge_ingress_message_broker_transport::VALIDATION_ERROR_PREFIX;

/// @covers: saf/validator_svc.rs::VALIDATION_ERROR_PREFIX — is a non-empty string.
#[test]
fn test_validation_error_prefix_const_is_nonempty_happy() {
    assert!(
        !VALIDATION_ERROR_PREFIX.is_empty(),
        "VALIDATION_ERROR_PREFIX must not be empty"
    );
}

/// @covers: saf/validator_svc.rs::VALIDATION_ERROR_PREFIX — contains the crate name segment.
#[test]
fn test_validation_error_prefix_const_contains_crate_name_error() {
    // The prefix must be recognisable; a wrong value would silently break
    // downstream error filtering that matches on this string.
    assert!(
        VALIDATION_ERROR_PREFIX.contains("message-broker"),
        "VALIDATION_ERROR_PREFIX must contain 'message-broker'; got: {VALIDATION_ERROR_PREFIX}"
    );
}

/// @covers: saf/validator_svc.rs::VALIDATION_ERROR_PREFIX — usable as an error message prefix.
#[test]
fn test_validation_error_prefix_const_can_prefix_an_error_message_edge() {
    let msg = format!("{VALIDATION_ERROR_PREFIX}: capacity must be > 0");
    assert!(msg.starts_with(VALIDATION_ERROR_PREFIX));
}
