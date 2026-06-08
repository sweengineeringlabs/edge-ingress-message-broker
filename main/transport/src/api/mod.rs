//! API layer — ingress message consumer port contracts.
pub(crate) mod default;
pub(crate) mod error;
pub(crate) mod nats;
pub(crate) mod traits;
pub(crate) mod types;

pub use error::{ConsumerError, ConsumerResult};
pub use nats::NatsMessageConsumer;
pub use traits::{MessageConsumer, Validator};
pub use types::{MessageConsumerConfig, NatsConsumerConfig, TransportSvc};
