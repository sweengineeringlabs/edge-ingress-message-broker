//! Integration tests — SAF NATS message consumer service constants.
//!
//! Verifies that `DEFAULT_NATS_URL` is well-formed and usable at call sites.

use swe_edge_ingress_message_broker_transport::DEFAULT_NATS_URL;

/// @covers: saf/nats_message_consumer_svc.rs::DEFAULT_NATS_URL — has the nats:// scheme.
#[test]
fn test_default_nats_url_const_has_nats_scheme_happy() {
    assert!(
        DEFAULT_NATS_URL.starts_with("nats://"),
        "DEFAULT_NATS_URL must use the nats:// scheme; got: {DEFAULT_NATS_URL}"
    );
}

/// @covers: saf/nats_message_consumer_svc.rs::DEFAULT_NATS_URL — points to port 4222.
#[test]
fn test_default_nats_url_const_uses_default_nats_port_error() {
    // The default NATS port is 4222; a misconfigured constant would cause
    // silent connection failures, so we assert the port explicitly.
    assert!(
        DEFAULT_NATS_URL.ends_with(":4222"),
        "DEFAULT_NATS_URL must use port 4222; got: {DEFAULT_NATS_URL}"
    );
}

/// @covers: saf/nats_message_consumer_svc.rs::DEFAULT_NATS_URL — is a non-empty string.
#[test]
fn test_default_nats_url_const_is_nonempty_edge() {
    assert!(
        !DEFAULT_NATS_URL.is_empty(),
        "DEFAULT_NATS_URL must not be empty"
    );
}
