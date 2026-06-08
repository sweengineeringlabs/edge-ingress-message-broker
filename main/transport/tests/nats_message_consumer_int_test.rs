//! Integration tests — NatsMessageConsumer trait bounds.

use swe_edge_ingress_message_broker_transport::{MessageConsumer, NatsMessageConsumer};

/// @covers: NatsMessageConsumer — is a supertrait of MessageConsumer
#[test]
fn test_nats_message_consumer_is_supertrait_of_message_consumer() {
    fn _assert_supertrait<T: NatsMessageConsumer>() {}
    // Compile-time check: NatsMessageConsumer implies MessageConsumer
    fn _bound_check<T: NatsMessageConsumer + MessageConsumer>() {}
}
