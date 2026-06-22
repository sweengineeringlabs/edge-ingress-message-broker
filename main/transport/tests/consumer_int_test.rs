//! Integration tests — MessageConsumer port via SAF factories.
#![allow(clippy::expect_used)]

#[cfg(feature = "in-memory")]
mod in_memory_tests {
    use futures::StreamExt;
    use swe_edge_ingress_message_broker_transport::TransportSvc;

    /// @covers: TransportSvc::default_consumer — health_check
    #[tokio::test]
    async fn test_default_consumer_health_check_returns_ok() {
        let c = TransportSvc::default_consumer();
        assert!(c.health_check().await.is_ok());
    }

    /// @covers: TransportSvc::default_consumer — subscribe
    #[tokio::test]
    async fn test_default_consumer_subscribe_returns_stream() {
        let c = TransportSvc::default_consumer();
        assert!(c.subscribe("test.topic").await.is_ok());
    }

    /// @covers: TransportSvc::default_consumer — stream is driveable
    #[tokio::test]
    async fn test_default_consumer_stream_is_driveable() {
        let c = TransportSvc::default_consumer();
        let mut stream = c.subscribe("events.test").await.expect("subscribe failed");
        match tokio::time::timeout(std::time::Duration::from_millis(10), stream.next()).await {
            Err(_) => {}
            Ok(item) => assert!(item.is_none(), "expected no message from empty stream"),
        }
    }

    /// @covers: TransportSvc::default_consumer — clone produces independent handle
    #[tokio::test]
    async fn test_default_consumer_clone_produces_independent_handle() {
        let c1 = TransportSvc::default_consumer();
        let c2 = c1.clone();
        assert!(c1.health_check().await.is_ok());
        assert!(c2.health_check().await.is_ok());
    }
}
