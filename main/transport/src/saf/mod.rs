//! SAF — ingress message consumer public factory surface.
mod transport_svc;

pub use crate::api::{ConsumerError, ConsumerResult};
pub use crate::api::{MessageConsumer, NatsMessageConsumer, Validator};
pub use crate::api::{MessageConsumerConfig, NatsConsumerConfig, TransportSvc};
pub use swe_edge_runtime_message_broker::{Message, MessageStream};
pub use transport_svc::{check_health, create_config_builder, subscribe_to, validate};

#[cfg(feature = "in-memory")]
pub use transport_svc::default_consumer;

#[cfg(feature = "nats")]
pub use transport_svc::nats_consumer;
