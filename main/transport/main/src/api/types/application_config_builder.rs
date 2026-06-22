//! Fluent builder for the ingress message-broker application configuration.

use crate::api::types::MessageConsumerConfig;

/// Fluent builder for assembling [`MessageConsumerConfig`] from application-level config.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_message_broker_transport::ApplicationConfigBuilder;
///
/// let cfg = ApplicationConfigBuilder::default().build();
/// assert_eq!(cfg.capacity, 1024);
/// ```
#[derive(Default)]
pub struct ApplicationConfigBuilder {
    capacity: Option<usize>,
}

impl ApplicationConfigBuilder {
    /// Override the channel capacity (default: 1024).
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

    /// Assemble the final [`MessageConsumerConfig`].
    pub fn build(self) -> MessageConsumerConfig {
        MessageConsumerConfig {
            capacity: self.capacity.unwrap_or(1024),
        }
    }
}
