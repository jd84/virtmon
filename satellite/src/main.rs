pub mod health_check {
    tonic::include_proto!("healthcheck");
}

use harvest::cpu::SysCpu;
use harvest::SystemData;
use health_check::health_check_server::{HealthCheck, HealthCheckServer};
use health_check::{Cpu, Empty, PingReply, PingRequest};
use tokio::sync::{mpsc, Mutex};
use tonic::{transport::Server, Request, Response, Status};

struct HealthCheckService {
    system: Mutex<SystemData>,
}

impl Default for HealthCheckService {
    fn default() -> Self {
        HealthCheckService {
            system: Mutex::new(SystemData::default()),
        }
    }
}

#[tonic::async_trait]
impl HealthCheck for HealthCheckService {
    type GetCpusStream = mpsc::Receiver<Result<Cpu, Status>>;

    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        self.system.lock().await.refresh();

        let reply = PingReply {
            name: "server01".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn get_cpus(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::GetCpusStream>, Status> {
        println!("Got a request={:?}", request);
        let (mut tx, rx) = mpsc::channel(4);

        let cpus = self
            .system
            .lock()
            .await
            .get_cpus()
            .iter()
            .map(|c| Cpu {
                name: c.name().to_string(),
                usage: c.usage(),
            })
            .collect::<Vec<_>>();

        tokio::spawn(async move {
            for cpu in &cpus[..] {
                tx.send(Ok(cpu.clone())).await.unwrap();
            }
        });

        Ok(Response::new(rx))
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
