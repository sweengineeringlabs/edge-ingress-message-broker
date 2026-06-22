//! `swe-edge-ingress-message-broker-transport` — opt-in ingress message consumer port.
//!
//! Wraps `swe-edge-runtime-message-broker` as a structured ingress port.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! swe-edge-ingress-message-broker-transport = { git = "...", features = ["in-memory"] }
//! ```
//!
//! ```rust,no_run
//! use swe_edge_ingress_message_broker_transport::{TransportSvc, MessageConsumer};
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Requires feature = "in-memory"
//! # #[cfg(feature = "in-memory")]
//! let consumer = TransportSvc::default_consumer();
//! # Ok(())
//! # }
//! ```
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

// Delegate public surface to saf/ as required by lib_delegates_to_saf.
pub use saf::*;

// Trait contracts — re-exported directly from api/ because saf/ cannot
// re-export traits ending in the forbidden suffixes (Consumer, Broker, etc.)
// per the saf_no_trait_reexport structural rule.
pub use api::{MessageConsumer, NatsMessageConsumer, Validator};

// Value types — re-exported from api/ because saf/ cannot re-export
// concrete types per the encapsulation.package_access_violation rule.
pub use api::{
    ApplicationConfigBuilder, CONSUMER_FUTURE_KIND, ConsumerBox, ConsumerError, ConsumerHealthResult,
    ConsumerResult, ConsumerStreamResult, HealthCheckFuture, MessageConsumerConfig, NatsConsumerConfig,
    SubscribeFuture, TransportSvc,
};
pub use swe_edge_runtime_message_broker::{Message, MessageStream};
