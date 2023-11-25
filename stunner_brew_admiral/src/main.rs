use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

async fn main() {

  let brokers = "kafka:9092";
  let topic = "sample_topic";
  let producer = FutureProducer::builder()
    .brokers(brokers)
    .build();

let message = "Test Message from Rust";

let record = FutureRecord::to::to(topic).key("test").payload(message);

match producer.send(record, Duration::from_secs(0)).await {
  Ok(_) => println!("Written: {}", message),
  Err(e) => eprintln!("Error writting message: {}", e),
  }
}
