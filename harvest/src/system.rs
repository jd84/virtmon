use crate::cpu::Cpu as SysCpu;
use crate::net::client::health_check::Cpu as NetCpu;
use crate::net::client::HealthCheckServiceClient;
use async_trait::async_trait;
use std::error::Error;
use sysinfo::{System, SystemExt};

#[async_trait]
pub trait SystemData {
    type Cpu;

    async fn refresh(&mut self) -> Result<(), Box<dyn Error>>;
    fn get_cpus(&self) -> &[Self::Cpu];
}

pub struct SystemManager<T> {
    system: T,
}

impl<T> SystemManager<T>
where
    T: SystemData,
{
    pub fn new(system: T) -> SystemManager<T> {
        SystemManager { system }
    }

    pub async fn refresh_all(&mut self) -> Result<(), Box<dyn Error>> {
        self.system.refresh().await?;
        Ok(())
    }

    pub fn get_cpus(&self) -> &[<T as SystemData>::Cpu] {
        self.system.get_cpus()
    }
}

pub struct LocalSystemData {
    system: System,
}

impl Default for LocalSystemData {
    fn default() -> LocalSystemData {
        let system = System::new_all();
        LocalSystemData { system }
    }
}

#[async_trait]
impl SystemData for LocalSystemData {
    type Cpu = sysinfo::Processor;

    async fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        self.system.refresh_all();
        Ok(())
    }

    fn get_cpus(&self) -> &[Self::Cpu] {
        self.system.get_processors()
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
}

#[async_trait]
impl SystemData for RemoteSystem {
    type Cpu = NetCpu;

    async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.ping().await?;
        self.system.rpc_get_cpus().await?;
        Ok(())
    }

    fn get_cpus(&self) -> &[Self::Cpu] {
        self.system.get_cpus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_system() {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let func = async {
            let mut sm = SystemManager::new(LocalSystemData::default());
            sm.refresh_all().await.unwrap();

            assert_eq!(true, sm.get_cpus().len() > 0);
        };

        rt.block_on(func);
    }
}
