# Changelog

All notable changes to `swe-edge-ingress-message-broker-transport` are recorded here.

## [Unreleased]

### Added

- Initial SEA-compliant crate scaffold: `api/`, `core/`, `saf/`, `spi/` layers.
- `MessageConsumer` trait with `subscribe` and `health_check` port contracts.
- `Validator` trait for config validation and adapter conversions.
- In-memory backend (`feature = "in-memory"`) backed by `InMemoryMessageBroker`.
- NATS backend (`feature = "nats"`) backed by `async-nats`.
- `TransportSvc` SAF factory: `default_consumer()` and `nats_consumer()`.
- Full SEA type surface: `ConsumerBox`, `ConsumerError`, `ConsumerResult`,
  `ConsumerStreamResult`, `ConsumerHealthResult`, `SubscribeFuture`,
  `HealthCheckFuture`, `MessageConsumerConfig`, `NatsConsumerConfig`,
  `ApplicationConfigBuilder`, `TransportSvc`.
