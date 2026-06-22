//! Typed configuration for the ingress message consumer.

use crate::api::{ApplicationConfigBuilder, Validator};

/// Runtime configuration for the ingress message consumer.
///
/// Loaded from the `[message_consumer]` section of `application.toml`.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_message_broker_transport::MessageConsumerConfig;
///
/// let cfg = MessageConsumerConfig::default();
/// assert_eq!(cfg.capacity, 1024);
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MessageConsumerConfig {
    /// Maximum in-memory channel capacity.
    pub capacity: usize,
}

impl Default for MessageConsumerConfig {
    fn default() -> Self {
        Self { capacity: 1024 }
    }
}

/// TOML section name for [`MessageConsumerConfig`] entries.
const MESSAGE_CONSUMER_SECTION: &str = "message_consumer";

impl swe_edge_configbuilder::ConfigSection for MessageConsumerConfig {
    fn section_name() -> &'static str {
        MESSAGE_CONSUMER_SECTION
    }
}

impl Validator for MessageConsumerConfig {
    fn validate(&self) -> Result<(), String> {
        if self.capacity == 0 {
            return Err("capacity must be > 0".into());
        }
        Ok(())
    }

    fn as_consumer_config(&self) -> Option<&MessageConsumerConfig> {
        Some(self)
    }

    fn to_config_builder(&self) -> Option<ApplicationConfigBuilder> {
        Some(ApplicationConfigBuilder::default().with_capacity(self.capacity))
    }
}
