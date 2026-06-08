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
mod gateway;
mod saf;

pub use gateway::*;
