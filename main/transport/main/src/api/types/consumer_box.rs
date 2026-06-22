//! Heap-allocated type-erased consumer handle.

use std::sync::Arc;

use crate::api::MessageConsumer;

/// A heap-allocated, type-erased handle to any [`MessageConsumer`] implementation.
///
/// Returned by SAF factory functions when the caller does not need to name the
/// concrete implementation. Callers hold this via `Arc` so it can be cheaply
/// cloned and shared across task boundaries.
///
/// # Examples
///
/// ```rust,no_run
/// # #[cfg(feature = "in-memory")]
/// # fn example() {
/// use swe_edge_ingress_message_broker_transport::{ConsumerBox, TransportSvc};
///
/// let consumer: ConsumerBox = TransportSvc::default_consumer();
/// # }
/// ```
pub type ConsumerBox = Arc<dyn MessageConsumer>;
