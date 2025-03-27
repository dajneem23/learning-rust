mod events;
mod analytics;

use futures::{Stream, StreamExt};
use tokio::time::{sleep, Duration};
use std::error::Error;


fn main() {
    println!("Hello, world!");
}





pub fn event_stream() -> impl Stream<Item = Event> {
    futures::stream::unfold(0, |count| async move {
        sleep(Duration::from_millis(150)).await; // Simulate delay between events
        let payload = format!("payload_{}", count);
        let event = Event::new(&payload);
        Some((event, count + 1))
    })
}

pub async fn process_event(event: Event) -> Result<Event, Box<dyn Error>> {
    tracing::info!("Processing event: {}", event.id);
    
    // Simulate heavy computation
    sleep(Duration::from_millis(100)).await;
    
    // Enrich the payload for demonstration
    let enriched_payload = format!("{} - enriched", event.payload);
    let mut processed_event = event.clone();
    processed_event.payload = enriched_payload;
    
    Ok(processed_event)
}

pub async fn run_event_pipeline() {
    let mut stream = event_stream();

    while let Some(event) = stream.next().await {
        // Spawn each event’s processing asynchronously
        tokio::spawn(async move {
            match process_event(event).await {
                Ok(processed) => {
                    tracing::info!("Processed event: {} with payload: {}",
                                    processed.id, processed.payload);
                    // Here, forward to the next stage (e.g., gRPC call to another service)
                },
                Err(e) => {
                    tracing::error!("Error processing event: {:?}", e);
                }
            }
        });
    }
}