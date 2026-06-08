//! Integration test — exercises `swe-edge-runtime-message-broker` dep directly.

#[cfg(feature = "in-memory")]
mod in_memory_tests {
    use swe_edge_runtime_message_broker::{MessageBroker, MessageBrokerFactory};

    /// @covers: MessageBrokerFactory::in_memory — health check
    #[tokio::test]
    async fn test_message_broker_dep_health_check_returns_ok() {
        assert!(MessageBrokerFactory::in_memory().health_check().await.is_ok());
    }

    /// @covers: MessageBrokerFactory::in_memory — subscribe
    #[tokio::test]
    async fn test_message_broker_dep_subscribe_returns_stream() {
        let broker = MessageBrokerFactory::in_memory();
        assert!(broker.subscribe("dep.test").await.is_ok());
    }
}
