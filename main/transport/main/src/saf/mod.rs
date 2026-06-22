//! SAF — ingress message consumer public factory surface.
//!
//! All api/ re-exports flow through `_svc.rs` files;
//! concrete types are re-exported from `lib.rs` directly from api/.
pub mod consumer;
mod message_consumer_svc;
mod nats_message_consumer_svc;
mod transport_svc;
mod types_svc;
mod validator_svc;

pub use message_consumer_svc::TOPIC_SEPARATOR;
pub use nats_message_consumer_svc::DEFAULT_NATS_URL;
pub use types_svc::DEFAULT_CONSUMER_CAPACITY;
pub use validator_svc::VALIDATION_ERROR_PREFIX;
