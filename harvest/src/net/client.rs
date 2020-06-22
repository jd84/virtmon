pub mod health_check {
    tonic::include_proto!("healthcheck");
}

use crate::cpu::SysCpu;
use health_check::health_check_client::HealthCheckClient;
use health_check::{Cpu, Empty, PingReply, PingRequest};
use tonic::transport::channel::Channel;
use tonic::{Response, Status};

impl SysCpu for Cpu {
    fn name(&self) -> &str {
        &self.name
    }
    fn usage(&self) -> f32 {
        self.usage
    }
}

pub struct HealthCheckServiceClient {
    client: HealthCheckClient<Channel>,
    cpus: Vec<Cpu>,
}

impl HealthCheckServiceClient {
    pub async fn new() -> Result<HealthCheckServiceClient, tonic::transport::Error> {
        let client = HealthCheckClient::connect("http://[::1]:50051").await?;
        Ok(HealthCheckServiceClient {
            client,
            cpus: Vec::new(),
        })
    }

    pub async fn ping(&mut self) -> Result<Response<PingReply>, Status> {
        let request = tonic::Request::new(PingRequest {
            name: "client01".to_string(),
        });

        self.client.ping(request).await
    }

    pub async fn rpc_get_cpus(&mut self) -> Result<(), Status> {
        let mut cpus = Vec::new();
        let mut stream = self
            .client
            .get_cpus(tonic::Request::new(Empty {}))
            .await?
            .into_inner();

        while let Some(cpu) = stream.message().await? {
            cpus.push(cpu);
        }
        self.cpus = cpus;
        Ok(())
    }

    pub fn get_cpus(&self) -> &[Cpu] {
        &self.cpus
    }
}
