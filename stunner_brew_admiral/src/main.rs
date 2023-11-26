use std::time::Duration;


use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};


#[tokio::main]
async fn main() {

  //let brokers = "kafka:9092";
  let topic = "sample_topic";
  let producer: &FutureProducer = &ClientConfig::new()
  .set("bootstrap.servers", "kafka:9092")  // Remplacement par ton adresse de serveur Kafka
  .set("message.timeout.ms", "5000")
  .create()
  .expect("Producer creation error");

  let futures = (0..5)
  .map(|i| async move {
      // The send operation on the topic returns a future, which will be
      // completed once the result or failure from Kafka is received.
      let delivery_status = producer
          .send(
              FutureRecord::to(topic)
                  .payload(&format!("Message {}", i))
                  .key(&format!("Key {}", i))
                  .headers(OwnedHeaders::new().add(
                      "header_key",
                      "header_value",
                  )),
              Duration::from_secs(0),
          )
          .await;

      // This will be executed when the result is received.
      println!("Delivery status for message {} received", i);
      delivery_status
  })
  .collect::<Vec<_>>();

// This loop will wait until all delivery statuses have been received.
  for future in futures {
    println!("Future completed. Result: {:?}", future.await);
  }
}
