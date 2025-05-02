use sysinfo::System;

pub struct CpuInfo<'a> {
    pub brand: &'a str,
    pub frequency: u64,
    pub usage: f32,
}

impl<'a> CpuInfo<'a> {
    pub fn new(system: &'a mut System, cpu_index: usize) -> CpuInfo<'a> {
        // CPU usage is calculated by diff. Needs to sleep for an instance to calculate.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        system.refresh_cpu_all();

        let cpu = &system.cpus()[cpu_index];

        CpuInfo {
            brand: cpu.brand(),
            frequency: cpu.frequency(),
            usage: cpu.cpu_usage(),
        }
    }
}
