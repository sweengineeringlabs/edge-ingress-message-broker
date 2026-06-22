//! NATS-backed `MessageConsumer`.

use std::sync::Arc;

use swe_edge_runtime_message_broker::{MessageBroker, MessageStream};

use futures::future::BoxFuture;

use crate::api::{ConsumerError, ConsumerHealthResult, ConsumerStreamResult, MessageConsumer, Validator};

/// Consumer backed by a NATS server via `async-nats`.
///
/// Construct via [`crate::saf::nats_consumer`].
#[derive(Clone)]
pub(crate) struct NatsMessageConsumer {
    inner: Arc<dyn MessageBroker>,
}

impl NatsMessageConsumer {
    pub(crate) fn new(broker: impl MessageBroker + 'static) -> Self {
        Self {
            inner: Arc::new(broker),
        }
    }
}

impl MessageConsumer for NatsMessageConsumer {
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

impl Validator for NatsMessageConsumer {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::BoxFuture;
    use swe_edge_runtime_message_broker::{BrokerError, Message};

    struct NatsMessageBrokerMock;
    impl MessageBroker for NatsMessageBrokerMock {
        fn publish<'a>(
            &'a self,
            _: &'a str,
            _: Message,
        ) -> BoxFuture<'a, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
        fn subscribe<'a>(
            &'a self,
            _: &'a str,
        ) -> BoxFuture<'a, Result<MessageStream, BrokerError>> {
            Box::pin(futures::future::ready(Ok(
                Box::pin(futures::stream::empty()) as MessageStream,
            )))
        }
        fn health_check(&self) -> BoxFuture<'_, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    /// @covers: new
    #[test]
    fn test_nats_message_consumer_new_accepts_any_broker() {
        let _ = NatsMessageConsumer::new(NatsMessageBrokerMock);
    }

    /// @covers: health_check
    #[test]
    fn test_nats_message_consumer_is_object_safe() {
        fn _assert(_: &dyn MessageConsumer) {}
    }
}
