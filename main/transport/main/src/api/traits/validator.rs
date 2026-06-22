//! `Validator` — port contract for configuration validation.

use crate::api::{ApplicationConfigBuilder, MessageConsumerConfig, NatsConsumerConfig, TransportSvc};

/// Validates a consumer configuration value before use.
pub trait Validator {
    /// Returns `Ok(())` when valid, or a human-readable error string.
    fn validate(&self) -> Result<(), String>;

    /// Cast self as a [`MessageConsumerConfig`] reference, if applicable.
    ///
    /// Returns `None` by default; override in [`MessageConsumerConfig`] to return `Some(self)`.
    fn as_consumer_config(&self) -> Option<&MessageConsumerConfig> {
        None
    }

    /// Cast self as a [`NatsConsumerConfig`] reference, if applicable.
    ///
    /// Returns `None` by default; override in [`NatsConsumerConfig`] to return `Some(self)`.
    fn as_nats_config(&self) -> Option<&NatsConsumerConfig> {
        None
    }

    /// Return the [`TransportSvc`] entry point, if this value can produce one.
    ///
    /// Returns `None` by default; override when the implementing type carries
    /// enough information to construct a `TransportSvc`.
    fn as_transport_svc(&self) -> Option<TransportSvc> {
        None
    }

    /// Build an [`ApplicationConfigBuilder`] seeded from this value, if applicable.
    ///
    /// Returns `None` by default; override in types that carry enough state to
    /// populate the builder (e.g. capacity-bearing config structs).
    fn to_config_builder(&self) -> Option<ApplicationConfigBuilder> {
        None
    }
}
