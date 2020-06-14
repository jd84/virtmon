use sysinfo::{Processor, ProcessorExt};

/// Represents a single cpu or in mordern systems a single core
pub struct Cpu {
    pub name: String,
    pub usage: f32,
}

/// Collection of `Cpu`s
pub struct Cpus {
    cpus: Vec<Cpu>,
}

impl Cpus {
    /// Create a collection from raw data
    pub fn from_raw(processors: &[Processor]) -> Cpus {
        let mut cpus = Vec::with_capacity(processors.len());
        for processor in processors {
            let cpu = Cpu {
                name: processor.get_name().to_string(),
                usage: processor.get_cpu_usage(),
            };
            cpus.push(cpu);
        }
        Cpus { cpus }
    }

    pub fn get_all(&self) -> &[Cpu] {
        &self.cpus
    }

    /// Refresh internal metrics direct from raw data
    pub fn refresh(&mut self, processors: &[Processor]) {
        for (i, processor) in processors.iter().enumerate() {
            if let Some(cpu) = self.cpus.get_mut(i) {
                cpu.usage = processor.get_cpu_usage();
            }
        }
    }
}
