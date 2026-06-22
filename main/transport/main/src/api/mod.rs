//! API layer — ingress message consumer port contracts.
mod default;
mod error;
mod nats;
mod traits;
mod types;

pub use error::{ConsumerError, ConsumerResult};
pub use nats::NatsMessageConsumer;
pub use traits::{MessageConsumer, Validator};
pub use types::{
    ApplicationConfigBuilder, ConsumerBox, CONSUMER_FUTURE_KIND, ConsumerHealthResult,
    ConsumerStreamResult, HealthCheckFuture, MessageConsumerConfig, NatsConsumerConfig,
    SubscribeFuture, TransportSvc,
};
