use std::env;
use sysinfo::{Cpu, CpuRefreshKind, RefreshKind, System};

mod cpu_info;
use cpu_info::CpuInfo;

fn main() {
    let mut system = System::new_all();
    let cpu_info = CpuInfo::new(&mut system, 0);
    println!(
        "CPU: {}\nFrequency: {}\nUsage: {}\n",
        cpu_info.brand, cpu_info.frequency, cpu_info.usage
    );
}

// fn main() {
//     let mut system =
//         System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
//     let wm = env::var("XDG_SESSION_DESKTOP").expect("Could not find window manager");

//     std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
//     system.refresh_cpu_all();
//     let cpu: &Cpu = &system.cpus().first().expect("Could not find CPU");

//     let cpu_usage: f32 = cpu.cpu_usage();

//     println!("WM {wm}\nCPU {}\nCPU Usage {}", &cpu.brand(), cpu_usage);
// }
