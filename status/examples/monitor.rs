fn main() {
    let metrics = system_monitor::run_system_monitor();
    print!("CPU: {:.1}% ({} MHz) || Temp: CPU {:.1}°C || GPU {:.1}°C || RAM: {:.2}/{:.2} GB ({:.1}%)",
        metrics.cpu_usage, metrics.cpu_freq, metrics.cpu_temp, metrics.gpu_temp, metrics.ram_usage,
        metrics.ram_total, metrics.ram_usage as f32 / metrics.ram_total as f32 * 100.0);
}