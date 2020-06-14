pub mod cpu;
pub mod process;

use cpu::{Cpu, Cpus};
use process::{Process, Processes};
use sysinfo::{System, SystemExt};

/// SystemData is a collection of all stats
pub struct SystemData {
    system: System,
    cpus: Cpus,
    processes: Processes,
}

impl SystemData {
    pub fn refresh(&mut self) {
        self.system.refresh_all();
        self.cpus.refresh(self.system.get_processors());
        self.processes.refresh(self.system.get_processes());
    }

    pub fn get_cpus(&self) -> &[Cpu] {
        &self.cpus.get_all()
    }

    pub fn get_processes(&self) -> Vec<&Process> {
        self.processes.get_sorted_cpu()
    }
}

impl Default for SystemData {
    fn default() -> SystemData {
        let mut system = System::new_all();
        system.refresh_all();

        let cpus = Cpus::from_raw(system.get_processors());
        let processes = Processes::from_raw(system.get_processes());

        SystemData {
            system,
            cpus,
            processes,
        }
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
