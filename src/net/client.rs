pub mod health_check {
    tonic::include_proto!("healthcheck");
}

use health_check::health_check_client::HealthCheckClient;
use health_check::{PingReply, PingRequest};
use tonic::{Response, Status};

pub struct HealthCheckServiceClient {
    client: HealthCheckClient<tonic::transport::channel::Channel>,
}

impl HealthCheckServiceClient {
    pub async fn new() -> Result<HealthCheckServiceClient, tonic::transport::Error> {
        let client = HealthCheckClient::connect("http://[::1]:50051").await?;
        Ok(HealthCheckServiceClient { client })
    }

    pub async fn ping(&mut self) -> Result<Response<PingReply>, Status> {
        let request = tonic::Request::new(PingRequest {
            name: "client01".to_string(),
        });

        self.client.ping(request).await
    }
}
