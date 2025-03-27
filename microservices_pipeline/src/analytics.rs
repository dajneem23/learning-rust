use tonic::{transport::Server, Request, Response, Status};
use analytics::analytics_service_server::{AnalyticsService, AnalyticsServiceServer};
use analytics::{EventRequest, EventResponse};
use std::time::Duration;

pub mod analytics {
    tonic::include_proto!("analytics");
}

#[derive(Default)]
pub struct AnalyticsServer {}

#[tonic::async_trait]
impl AnalyticsService for AnalyticsServer {
    async fn process_event(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        let req = request.into_inner();
        tracing::info!("Received event via gRPC: {}", req.id);
        
        // Simulate processing delay
        sleep(Duration::from_millis(50)).await;
        
        let reply = EventResponse {
            success: true,
            message: format!("Event {} processed", req.id),
        };
        Ok(Response::new(reply))
    }
}

pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let analytics_server = AnalyticsServer::default();

    Server::builder()
        .add_service(AnalyticsServiceServer::new(analytics_server))
        .serve(addr)
        .await?;
    Ok(())
}

pub async fn send_event_via_grpc(event: Event) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AnalyticsServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(EventRequest {
        id: event.id.to_string(),
        timestamp: event.timestamp,
        payload: event.payload,
    });
    let response = client.process_event(request).await?;
    tracing::info!("gRPC Response: {:?}", response.into_inner());
    Ok(())
}