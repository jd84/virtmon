pub mod health_check {
    tonic::include_proto!("healthcheck");
}

use health_check::health_check_server::{HealthCheck, HealthCheckServer};
use health_check::{PingReply, PingRequest};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
struct HealthCheckService {}

#[tonic::async_trait]
impl HealthCheck for HealthCheckService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        println!("Got a request={:?}", request);

        let reply = PingReply {
            name: "server01".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let health_check = HealthCheckService::default();

    Server::builder()
        .add_service(HealthCheckServer::new(health_check))
        .serve(addr)
        .await?;

    Ok(())
}
