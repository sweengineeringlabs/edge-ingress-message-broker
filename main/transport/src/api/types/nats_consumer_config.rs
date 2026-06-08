//! NATS-specific connection configuration for the ingress message consumer.

/// Connection configuration for the NATS-backed consumer.
///
/// Passed to [`crate::TransportSvc::nats_consumer`] to establish a connection.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_message_broker_transport::NatsConsumerConfig;
///
/// let cfg = NatsConsumerConfig { url: "nats://localhost:4222".into() };
/// assert_eq!(cfg.url, "nats://localhost:4222");
/// ```
#[derive(Debug, Clone)]
pub struct NatsConsumerConfig {
    /// NATS server URL, e.g. `"nats://localhost:4222"`.
    pub url: String,
}
