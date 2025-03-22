use std::thread;
use std::time::Duration;

mod module {
    pub mod m1 {
        pub mod chip;
        pub mod sources;
    }
    pub mod disk {
        pub mod disk;
    }
    pub mod net {
        pub mod network;
    }
}

use module::m1::chip::Sampler;

#[repr(C)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub cpu_freq: u64,
    pub cpu_temp: f64,
    pub gpu_temp: f64,
    pub ram_usage: u64,
    pub ram_total: u64,
}

#[no_mangle]
pub extern "C" fn run_system_monitor() -> SystemMetrics {
    let mut sampler = Sampler::new().expect("Falha ao inicializar Sampler");
    let metrics = sampler.get_metrics(1000).expect("Falha ao obter métricas do sistema");
    
    return SystemMetrics {
        cpu_usage: (metrics.ecpu_usage.1 as f64) * 100.0,
        cpu_freq: metrics.ecpu_usage.0 as u64,
        cpu_temp: metrics.temp.cpu_temp as f64,
        gpu_temp: metrics.temp.gpu_temp as f64,
        ram_usage: metrics.memory.ram_usage as u64,
        ram_total: metrics.memory.ram_total as u64,
    };

}