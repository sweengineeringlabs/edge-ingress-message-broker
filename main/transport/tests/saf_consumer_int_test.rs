//! Integration tests — SAF public API for the ingress message consumer.

#[cfg(feature = "in-memory")]
mod in_memory_tests {
    use swe_edge_ingress_message_broker_transport::{
        check_health, default_consumer, subscribe_to, validate, MessageConsumer, TransportSvc,
        Validator,
    };

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    /// @covers: default_consumer
    #[tokio::test]
    async fn test_default_consumer_saf_factory_returns_healthy_consumer() {
        let c = default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    /// @covers: subscribe_to
    #[tokio::test]
    async fn test_subscribe_to_returns_stream_for_default_consumer() {
        let c = default_consumer();
        assert!(subscribe_to(&c, "events.test").await.is_ok());
    }

    /// @covers: check_health
    #[tokio::test]
    async fn test_check_health_returns_ok_for_default_consumer() {
        let c = default_consumer();
        assert!(check_health(&c).await.is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_returns_ok_for_always_valid() {
        assert!(validate(&AlwaysValid).is_ok());
    }

    /// @covers: TransportSvc::default_consumer
    #[tokio::test]
    async fn test_transport_svc_default_consumer_returns_healthy_consumer() {
        let c = TransportSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }
}
