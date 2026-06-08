//! Example: subscribe to a topic using the in-memory consumer.

#[cfg(feature = "in-memory")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use futures::StreamExt;
    use swe_edge_ingress_message_broker_transport::{MessageConsumer, TransportSvc};

    let consumer = TransportSvc::default_consumer();
    let mut stream = consumer.subscribe("orders.created").await?;
    println!("Subscribed to orders.created — waiting for messages (Ctrl+C to stop).");
    while let Some(msg) = stream.next().await {
        println!("Received: {:?}", msg);
    }
    Ok(())
}

#[cfg(not(feature = "in-memory"))]
fn main() {
    eprintln!("Run with --features in-memory");
}
