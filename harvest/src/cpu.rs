use sysinfo::{Processor, ProcessorExt};

pub trait SysCpu {
    fn name(&self) -> &str;
    fn usage(&self) -> f32;
}

/// Represents a single cpu or in mordern systems a single core
pub struct Cpu {
    name: String,
    usage: f32,
}

impl Cpu {
    /// Create a collection from raw data
    pub fn from_raw(p: &Processor) -> Cpu {
        Cpu {
            name: p.get_name().into(),
            usage: p.get_cpu_usage(),
        }
    }
}

impl SysCpu for Cpu {
    fn name(&self) -> &str {
        &self.name
    }

    fn usage(&self) -> f32 {
        self.usage
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
        let cpus = sys
            .get_processors()
            .iter()
            .map(|p| Cpu::from_raw(&p))
            .collect::<Vec<_>>();

        assert_eq!(true, cpus.len() > 0);
    }
}
