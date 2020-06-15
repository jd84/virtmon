use sysinfo::{Processor, ProcessorExt};

/// Represents a single cpu or in mordern systems a single core
pub struct Cpu<'a> {
    raw: &'a Processor,
    name: String,
}

impl<'a> Cpu<'a> {
    /// Create a collection from raw data
    pub fn from_raw(processors: &'a Processor) -> Cpu<'a> {
        Cpu {
            raw: processors,
            name: processors.get_name().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn usage(&self) -> f32 {
        self.raw.get_cpu_usage()
    }
}
