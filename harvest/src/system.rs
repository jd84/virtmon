use crate::cpu::Cpu as SysCpu;
use crate::net::client::health_check::Cpu as NetCpu;
use crate::net::client::HealthCheckServiceClient;
use async_trait::async_trait;
use sysinfo::{System, SystemExt};

#[async_trait]
pub trait SystemData {
    type Cpu;

    async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_cpus(&self) -> &[Self::Cpu];
}

pub struct LocalSystemData {
    system: System,
    cpus: Vec<SysCpu>,
}

#[async_trait]
impl SystemData for LocalSystemData {
    type Cpu = SysCpu;

    async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.refresh_all();

        self.cpus = self
            .system
            .get_processors()
            .iter()
            .map(SysCpu::from_raw)
            .collect::<Vec<_>>();
        Ok(())
    }

    fn get_cpus(&self) -> &[Self::Cpu] {
        &self.cpus
    }
}

pub struct RemoteSystem {
    system: HealthCheckServiceClient,
}

impl RemoteSystem {
    pub async fn new() -> RemoteSystem {
        RemoteSystem {
            system: HealthCheckServiceClient::new().await.unwrap(),
        }
    }

    pub async fn refresh_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.ping().await?;
        self.system.rpc_get_cpus().await?;
        Ok(())
    }
}

#[async_trait]
impl SystemData for RemoteSystem {
    type Cpu = NetCpu;

    async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.rpc_get_cpus().await?;
        Ok(())
    }

    fn get_cpus(&self) -> &[Self::Cpu] {
        unimplemented!();
    }
}
