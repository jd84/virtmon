use slab::Slab;
use std::collections::HashMap;
use sysinfo::Pid;
use sysinfo::{Process as SysProcess, ProcessExt};

pub struct Process {
    pub pid: Pid,
    pub name: String,
    pub cmd: Vec<String>,
    pub status: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

impl From<&SysProcess> for Process {
    fn from(process: &SysProcess) -> Process {
        let cmd = process.cmd().iter().cloned().collect::<Vec<_>>();

        Process {
            pid: process.pid(),
            name: process.name().to_string(),
            cmd,
            status: process.status().to_string().to_owned(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
        }
    }
}

pub struct Processes {
    processes: Slab<Process>,
    process_pids: HashMap<Pid, usize>,
}

impl Processes {
    pub fn from_raw(processes: &HashMap<Pid, SysProcess>) -> Processes {
        let mut procs = Slab::new();
        let mut process_pids = HashMap::new();

        for (pid, p) in processes {
            let id = procs.insert(p.into());
            process_pids.insert(*pid, id);
        }

        Processes {
            processes: procs,
            process_pids,
        }
    }

    pub fn refresh(&mut self, processes: &HashMap<Pid, SysProcess>) {
        for (pid, p) in processes {
            if let Some(id) = self.process_pids.get(pid) {
                if let Some(process) = self.processes.get_mut(*id as usize) {
                    process.cpu_usage = p.cpu_usage();
                    process.memory = p.memory();
                }
            } else {
                let id = self.processes.insert(p.into());
                self.process_pids.insert(*pid, id);
            }
        }

        let mut remove_pids = Vec::new();
        for pid in self.process_pids.keys() {
            if !processes.contains_key(&pid) {
                remove_pids.push(*pid);
            }
        }
        for pid in remove_pids {
            if let Some(did) = self.process_pids.remove(&pid) {
                self.processes.remove(did);
            }
        }
    }

    pub fn get_all(&self) -> Vec<&Process> {
        self.processes.iter().map(|p| p.1).collect::<Vec<_>>()
    }

    pub fn get_sorted_cpu(&self) -> Vec<&Process> {
        let mut processes = self.get_all();
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes
    }
}
