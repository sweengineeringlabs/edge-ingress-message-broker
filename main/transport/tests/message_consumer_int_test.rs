//! Integration tests — MessageConsumer trait contract.

#[cfg(feature = "in-memory")]
mod in_memory_tests {
    use swe_edge_ingress_message_broker_transport::{default_consumer, MessageConsumer};

    /// @covers: MessageConsumer::health_check — health_check contract
    #[tokio::test]
    async fn test_message_consumer_health_check_succeeds_on_in_memory() {
        let c = default_consumer();
        assert!(
            c.health_check().await.is_ok(),
            "in-memory consumer health_check must succeed"
        );
    }

    /// @covers: MessageConsumer::subscribe — subscribe contract
    #[tokio::test]
    async fn test_message_consumer_subscribe_returns_ok_for_valid_topic() {
        let c = default_consumer();
        assert!(
            c.subscribe("test.topic").await.is_ok(),
            "subscribe must succeed on a valid topic"
        );
    }

    /// @covers: MessageConsumer::subscribe — multiple topics are independent
    #[tokio::test]
    async fn test_message_consumer_subscribe_multiple_topics_are_independent() {
        let c = default_consumer();
        assert!(c.subscribe("a.topic").await.is_ok());
        assert!(c.subscribe("b.topic").await.is_ok());
    }
}
