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

use module::disk::disk::DiskSampler;
use module::net::network::NetworkSampler;
use module::m1::chip::Sampler;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// struct Settings {
//     update_interval_secs: u64,
//     show_cpu: bool,
//     show_memory: bool,
//     show_disk: bool,
// }

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// struct Config {
//     settings: Settings,
// }

// Declarações das funções C
// extern "C" {
//     fn config_load(path: *const c_char) -> *mut Config;
//     fn config_free(config: *mut Config);
//     fn config_default() -> Config;
// }

fn main() {
    let mut disk_sampler = DiskSampler::new().expect("Falha ao inicializar DiskSampler");
    let mut network_sampler = NetworkSampler::new().expect("Falha ao inicializar NetworkSampler");
    let mut sampler = Sampler::new().expect("Falha ao inicializar Sampler");


        // Carrega a configuração ou usa o padrão
        // let config_path = CString::new("config.txt").expect("Falha ao criar CString");
        // let config_ptr = config_load(config_path.as_ptr());
        // let config = if config_ptr.is_null() {
        //     println!("Falha ao carregar config, usando padrão");
        //     config_default()
        // } else {
        //     (*config_ptr).clone()
        // };

        // println!(
        //     "Configuração: intervalo={}s, cpu={}, mem={}, disk={}",
        //     config.settings.update_interval_secs,
        //     config.settings.show_cpu,
        //     config.settings.show_memory,
        //     config.settings.show_disk
        // );

    // Loop principal
    loop {
        // Obter métricas dos samplers
        let disk_metrics = disk_sampler.get_metrics().expect("Falha ao obter métricas de disco");
        let network_metrics = network_sampler.get_metrics().expect("Falha ao obter métricas de rede");
        let metrics = sampler.get_metrics(1000).expect("Falha ao obter métricas do sistema"); // 1000ms de duração

        // Imprimir as métricas no terminal
        println!("Estatísticas do Sistema:");
        println!(
            "CPU: {:.1}% ({} MHz) | Temp: CPU {:.1}°C | GPU {:.1}°C | RAM: {:.2} GB | Disk: {:.2}/{:.2} GB | Net: {:.2}/{:.2}",
            metrics.ecpu_usage.1 * 100.0,
            metrics.ecpu_usage.0,
            metrics.temp.cpu_temp,
            metrics.temp.gpu_temp,
            metrics.memory.ram_usage as f64 / 1024.0 / 1024.0 / 1024.0,
            disk_metrics.used_space as f64 / 1024.0 / 1024.0 / 1024.0,
            disk_metrics.total_space as f64 / 1024.0 / 1024.0 / 1024.0,
            network_metrics.download_bytes as f64 / 1024.0 / 1024.0,
            network_metrics.upload_bytes as f64 / 1024.0 / 1024.0,
        );

        // Aguarda o intervalo definido
        thread::sleep(Duration::from_secs(1));
    }

}