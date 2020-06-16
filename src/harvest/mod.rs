pub mod cpu;
pub mod process;

use crate::net::client::HealthCheckServiceClient;
use cpu::Cpu;
use process::Process;
use sysinfo::{System, SystemExt};

/// SystemData is a collection of all stats
pub struct SystemData {
    system: System,
}

impl SystemData {
    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    pub fn get_cpus(&self) -> Vec<Cpu> {
        self.system
            .get_processors()
            .iter()
            .map(|p| Cpu::from_raw(p))
            .collect::<Vec<_>>()
    }

    pub fn get_processes(&self) -> Vec<Process> {
        let mut procs = self
            .system
            .get_processes()
            .iter()
            .map(|(_pid, p)| Process::from_raw(p))
            .collect::<Vec<_>>();
        procs.sort_by(|a, b| b.cpu_usage_raw().partial_cmp(&a.cpu_usage_raw()).unwrap());
        procs
    }
}

impl Default for SystemData {
    fn default() -> SystemData {
        let mut system = System::new_all();
        system.refresh_all();

        SystemData { system }
    }
}

pub struct RemoteSystemData {
    system: HealthCheckServiceClient,
}

impl RemoteSystemData {
    pub async fn new() -> RemoteSystemData {
        RemoteSystemData {
            system: HealthCheckServiceClient::new().await.unwrap(),
        }
    }

    pub async fn refresh(&mut self) {
        self.system.ping().await.unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_systemdata() {
        let sys_data = SystemData::default();
        let cpus = sys_data.get_cpus();
        assert_eq!(true, cpus.len() > 0);
    }
}
