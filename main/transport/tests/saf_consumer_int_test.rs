//! Integration tests — SAF public API for the ingress message consumer.

#[cfg(feature = "in-memory")]
mod in_memory_tests {
    use swe_edge_ingress_message_broker_transport::{MessageConsumer, TransportSvc, Validator};

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    /// @covers: TransportSvc::default_consumer
    #[tokio::test]
    async fn test_default_consumer_saf_factory_returns_healthy_consumer() {
        let c = TransportSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    /// @covers: TransportSvc::subscribe_to
    #[tokio::test]
    async fn test_subscribe_to_returns_stream_for_default_consumer() {
        let c = TransportSvc::default_consumer();
        assert!(TransportSvc::subscribe_to(&c, "events.test").await.is_ok());
    }

    /// @covers: TransportSvc::check_health
    #[tokio::test]
    async fn test_check_health_returns_ok_for_default_consumer() {
        let c = TransportSvc::default_consumer();
        assert!(TransportSvc::check_health(&c).await.is_ok());
    }

    /// @covers: TransportSvc::validate
    #[test]
    fn test_validate_returns_ok_for_always_valid() {
        assert!(TransportSvc::validate(&AlwaysValid).is_ok());
    }

    /// @covers: TransportSvc::create_config_builder
    #[test]
    fn test_create_config_builder_returns_builder() {
        let _builder = TransportSvc::create_config_builder();
    }
}
