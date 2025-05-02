use sysinfo::System;

mod color_info;
mod cpu_info;
mod wm_info;
use color_info::print_colors;
use cpu_info::CpuInfo;
use wm_info::WmInfo;

fn main() {
    let mut system = System::new_all();
    let cpu_info = CpuInfo::new(&mut system, 0);
    let wm_info = WmInfo::new();

    println!(
        "WM: {}\nCPU: {}\nFrequency: {}\nUsage: {}\n",
        wm_info.name, cpu_info.brand, cpu_info.frequency, cpu_info.usage
    );

    print_colors("FETCH ".to_string(), true);
}
