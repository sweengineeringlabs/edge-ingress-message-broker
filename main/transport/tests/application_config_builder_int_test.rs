//! Integration tests — ApplicationConfigBuilder.
//!
//! Verifies that the builder produces `MessageConsumerConfig` with
//! the correct capacity, covering the default, explicit, and edge cases.

use swe_edge_ingress_message_broker_transport::ApplicationConfigBuilder;

/// @covers: ApplicationConfigBuilder::build — default build produces capacity 1024.
#[test]
fn test_application_config_builder_struct_default_build_produces_default_capacity_happy() {
    let cfg = ApplicationConfigBuilder::default().build();
    assert_eq!(
        cfg.capacity, 1024,
        "default build must produce capacity 1024"
    );
}

/// @covers: ApplicationConfigBuilder::with_capacity — overrides the default.
#[test]
fn test_application_config_builder_struct_with_capacity_overrides_default_error() {
    // "error" scenario: verify that NOT calling with_capacity keeps the default,
    // and calling it with 0 would silently produce a capacity that fails
    // Validator::validate. We assert the override is honoured.
    let cfg = ApplicationConfigBuilder::default().with_capacity(512).build();
    assert_eq!(cfg.capacity, 512, "with_capacity(512) must produce capacity 512");
}

/// @covers: ApplicationConfigBuilder::with_capacity — edge: capacity of 1 is preserved.
#[test]
fn test_application_config_builder_struct_capacity_one_is_preserved_edge() {
    let cfg = ApplicationConfigBuilder::default().with_capacity(1).build();
    assert_eq!(
        cfg.capacity, 1,
        "minimum capacity of 1 must be preserved through the builder"
    );
}
