use std::ffi::CString;
use std::os::raw::c_char;
use std::thread;
use std::time::Duration;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Settings {
    update_interval_secs: u64,
    show_cpu: bool,
    show_memory: bool,
    show_disk: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Config {
    settings: Settings,
}

#[repr(C)]
#[derive(Debug)]
struct HardwareStats {
    cpu_usage: u32,
    memory_used: f64,
    memory_total: f64,
    network_in: f64,
    network_out: f64,
    disk_used: f64,
    disk_total: f64,
}

// Declarações das funções C
extern "C" {
    fn config_load(path: *const c_char) -> *mut Config;
    fn config_free(config: *mut Config);
    fn config_default() -> Config;
    fn get_hardware_stats(config: Config, stats: *mut HardwareStats) -> i32;
    fn init_hardware_stats() -> HardwareStats;
}

fn main() {

    unsafe {
        // Carrega a configuração ou usa o padrão
        let config_path = CString::new("config.txt").expect("Falha ao criar CString");
        let config_ptr = config_load(config_path.as_ptr());
        let config = if config_ptr.is_null() {
            println!("Falha ao carregar config, usando padrão");
            config_default()
        } else {
            (*config_ptr).clone()
        };

        println!(
            "Configuração: intervalo={}s, cpu={}, mem={}, disk={}",
            config.settings.update_interval_secs,
            config.settings.show_cpu,
            config.settings.show_memory,
            config.settings.show_disk
        );

        // Loop principal
        loop {
            let mut stats = init_hardware_stats();
            let result = get_hardware_stats(config, &mut stats);

            if result == 0 {
                println!("Estatísticas de Hardware:");
                if config.settings.show_cpu {
                    println!("Uso de CPU: {}%", stats.cpu_usage);
                }
                if config.settings.show_memory {
                    println!("Memória: {:.1} GB / {:.1} GB", stats.memory_used, stats.memory_total);
                }
                if config.settings.show_disk {
                    println!("Disco: {:.1} GB / {:.1} GB", stats.disk_used, stats.disk_total);
                }
                println!("Rede: In {:.1} MB, Out {:.1} MB", stats.network_in, stats.network_out);
            } else {
                println!("Erro ao obter estatísticas de hardware");
                break;
            }

            // Aguarda o intervalo definido
            thread::sleep(Duration::from_secs(config.settings.update_interval_secs));
        }

        // Libera a memória se config foi carregada do arquivo
        if !config_ptr.is_null() {
            config_free(config_ptr);
        }
    }
}