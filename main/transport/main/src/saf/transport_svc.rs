//! SAF transport facade — factory methods on [`TransportSvc`].

use crate::api::{HealthCheckFuture, MessageConsumer, SubscribeFuture, TransportSvc, Validator};

impl TransportSvc {
    /// Return a [`ConfigBuilder`](swe_edge_configbuilder::ConfigBuilderImpl) pre-seeded with this crate's name and version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_message_broker_transport::TransportSvc;
    ///
    /// let builder = TransportSvc::create_config_builder();
    /// let _loader = builder.build_loader();
    /// ```
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Validate any type implementing [`Validator`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_message_broker_transport::{TransportSvc, MessageConsumerConfig};
    ///
    /// let cfg = MessageConsumerConfig::default();
    /// assert!(TransportSvc::validate(&cfg).is_ok());
    /// ```
    pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }

    /// Subscribe to `topic` using any [`MessageConsumer`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    /// use swe_edge_ingress_message_broker_transport::{TransportSvc, MessageConsumer};
    ///
    /// # async fn example(consumer: Arc<dyn MessageConsumer>) {
    /// let _stream = TransportSvc::subscribe_to(consumer.as_ref(), "orders.created").await;
    /// # }
    /// ```
    pub fn subscribe_to<'a>(
        consumer: &'a dyn MessageConsumer,
        topic: &'a str,
    ) -> SubscribeFuture<'a> {
        consumer.subscribe(topic)
    }

    /// Run a health check on any [`MessageConsumer`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    /// use swe_edge_ingress_message_broker_transport::{TransportSvc, MessageConsumer};
    ///
    /// # async fn example(consumer: Arc<dyn MessageConsumer>) {
    /// let _ = TransportSvc::check_health(consumer.as_ref()).await;
    /// # }
    /// ```
    pub fn check_health(consumer: &dyn MessageConsumer) -> HealthCheckFuture<'_> {
        consumer.health_check()
    }

    /// Construct an in-memory consumer backed by a tokio broadcast channel.
    ///
    /// Returns an `Arc<dyn MessageConsumer>` ([`ConsumerBox`](crate::ConsumerBox))
    /// that can be cheaply cloned and shared across async tasks.
    ///
    /// Requires the `in-memory` feature.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "in-memory")]
    /// # fn example() {
    /// use swe_edge_ingress_message_broker_transport::{ConsumerBox, TransportSvc};
    ///
    /// let consumer: ConsumerBox = TransportSvc::default_consumer();
    /// # }
    /// ```
    #[cfg(feature = "in-memory")]
    pub fn default_consumer() -> crate::api::ConsumerBox {
        use std::sync::Arc;
        Arc::new(crate::core::DefaultMessageConsumer::new())
    }

    /// Connect to a NATS server and return a consumer handle.
    ///
    /// Returns a [`ConsumerBox`](crate::ConsumerBox) (`Arc<dyn MessageConsumer>`)
    /// that can be cheaply cloned and shared across async tasks.
    ///
    /// Requires the `nats` feature.
    ///
    /// # Errors
    /// Returns [`ConsumerError::Connection`] when the server is unreachable.
    #[cfg(feature = "nats")]
    pub async fn nats_consumer(
        url: &str,
    ) -> Result<crate::api::ConsumerBox, crate::api::ConsumerError> {
        use std::sync::Arc;
        use swe_edge_runtime_message_broker::MessageBrokerFactory;
        let broker = MessageBrokerFactory::nats(url)
            .await
            .map_err(crate::api::ConsumerError::from)?;
        Ok(Arc::new(crate::core::NatsMessageConsumer::new(broker)))
    }
}
