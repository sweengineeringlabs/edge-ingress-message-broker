//! Integration tests — NatsConsumerConfig.

use swe_edge_ingress_message_broker_transport::NatsConsumerConfig;

/// @covers: api/types/nats_consumer_config.rs — fields are accessible and correct
#[test]
fn test_nats_consumer_config_url_is_accessible() {
    let cfg = NatsConsumerConfig { url: "nats://localhost:4222".into() };
    assert_eq!(cfg.url, "nats://localhost:4222");
}

/// @covers: api/types/nats_consumer_config.rs — clone produces independent instance
#[test]
fn test_nats_consumer_config_clone_is_independent() {
    let original = NatsConsumerConfig { url: "nats://a:4222".into() };
    let mut cloned = original.clone();
    cloned.url = "nats://b:4222".into();
    assert_eq!(original.url, "nats://a:4222");
}
