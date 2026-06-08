//! Integration tests — TransportSvc facade type.

use swe_edge_ingress_message_broker_transport::TransportSvc;

/// @covers: api/types/transport_svc.rs — TransportSvc type is exported
#[test]
fn test_transport_svc_is_exported() {
    let _ = std::any::type_name::<TransportSvc>();
}

/// @covers: TransportSvc::create_config_builder — returns usable builder
#[test]
fn test_transport_svc_create_config_builder_returns_builder() {
    let _builder = TransportSvc::create_config_builder();
}
