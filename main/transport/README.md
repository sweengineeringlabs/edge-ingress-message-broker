# swe-edge-ingress-message-broker-transport

Ingress port crate for inbound message consumption.
Wraps `swe-edge-runtime-message-broker` as a typed SEA port contract.

## Features

| Feature      | Backend                                    |
|--------------|--------------------------------------------|
| `in-memory`  | `InMemoryMessageBroker` (tokio broadcast)  |
| `nats`       | `NatsMessageBroker` (async-nats)           |

## Quick Start

```toml
[dependencies]
swe-edge-ingress-message-broker-transport = { path = ".", features = ["in-memory"] }
```

```rust,no_run
use swe_edge_ingress_message_broker_transport::{TransportSvc, MessageConsumer};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
# #[cfg(feature = "in-memory")]
let consumer = TransportSvc::default_consumer();
let stream = consumer.subscribe("orders.created").await?;
# Ok(())
# }
```

## Architecture

```
api/     Public traits and types (port contract)
core/    Concrete implementations (pub(crate) only)
saf/     SAF factories — the only public export surface
spi/     Extension hooks for downstream consumers
```

Consumers call `TransportSvc` factory functions in `saf/` and program against
the `MessageConsumer` trait from `api/`. Concrete types in `core/` are never
exposed directly.
