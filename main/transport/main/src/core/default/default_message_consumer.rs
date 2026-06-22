//! In-memory `MessageConsumer` backed by `MessageBrokerFactory::in_memory()`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_runtime_message_broker::{MessageBroker, MessageBrokerFactory};

use crate::api::{ConsumerError, ConsumerHealthResult, ConsumerStreamResult, MessageConsumer, Validator};

/// In-process consumer backed by a tokio broadcast channel.
///
/// Suitable for single-process deployments, integration tests, and development.
#[derive(Clone)]
pub(crate) struct DefaultMessageConsumer {
    inner: Arc<dyn MessageBroker>,
}

impl DefaultMessageConsumer {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(MessageBrokerFactory::in_memory()),
        }
    }
}

impl MessageConsumer for DefaultMessageConsumer {
    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, ConsumerStreamResult> {
        Box::pin(async move {
            self.inner
                .subscribe(topic)
                .await
                .map_err(ConsumerError::from)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, ConsumerHealthResult> {
        Box::pin(async move { self.inner.health_check().await.map_err(ConsumerError::from) })
    }
}

impl Validator for DefaultMessageConsumer {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_default_message_consumer_new_is_constructible() {
        let _ = DefaultMessageConsumer::new();
    }

    /// @covers: health_check
    #[tokio::test]
    async fn test_default_message_consumer_health_check_returns_ok() {
        assert!(DefaultMessageConsumer::new().health_check().await.is_ok());
    }

    /// @covers: subscribe
    #[tokio::test]
    async fn test_default_message_consumer_subscribe_returns_stream() {
        let c = DefaultMessageConsumer::new();
        assert!(c.subscribe("test.topic").await.is_ok());
    }
}
