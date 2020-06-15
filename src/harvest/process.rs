use sysinfo::{Process as SysProcess, ProcessExt};

/// Process is the adapter class for `sysinfo::Process`
pub struct Process<'a> {
    raw: &'a SysProcess,
    pid: String,
    status: String,
    cpu_usage: String,
    mem_usage: String,
}

impl<'a> Process<'a> {
    pub(crate) fn from_raw(p: &'a SysProcess) -> Process<'a> {
        Process {
            raw: p,
            pid: p.pid().to_string(),
            status: p.status().to_string().to_owned(),
            cpu_usage: p.cpu_usage().to_string(),
            mem_usage: p.memory().to_string(),
        }
    }

    pub fn pid(&self) -> &str {
        &self.pid
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn name(&self) -> &str {
        &self.raw.name()
    }

    pub fn cpu_usage(&self) -> &str {
        &self.cpu_usage
    }

    pub fn cpu_usage_raw(&self) -> f32 {
        self.raw.cpu_usage()
    }

    pub fn mem_usage(&self) -> &str {
        &self.mem_usage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::{System, SystemExt};

    #[test]
    fn test_cpu() {
        let mut sys = System::new_all();
        sys.refresh_all();
        let procs = sys
            .get_processes()
            .iter()
            .map(|(_pid, p)| Process::from_raw(p))
            .collect::<Vec<_>>();

        assert_eq!(true, procs.len() > 0);
    }
}
