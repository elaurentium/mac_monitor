use serde::Serialize;
use sysinfo::{Disks, System};
use std::io;
use std::process::Command;

type WithError<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default, Serialize)]
pub struct DiskMetrics {
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
}

#[allow(dead_code)]
pub struct DiskSampler {
    last_read: u64,
    last_write: u64,
    last_time: std::time::Instant,
}

impl DiskSampler {
    pub fn new() -> WithError<Self> {
        Ok(DiskSampler {
            last_read: 0,
            last_write: 0,
            last_time: std::time::Instant::now(),
        })
    }

    fn get_disk_io(&self) -> WithError<(u64, u64)> {
        let output = Command::new("iostat")
            .args(&["-d", "-I"])  // Disk stats
            .output()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        if !output.status.success() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "iostat command failed",
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut total_read = 0;
        let mut total_write = 0;

        // Skip header
        for line in stdout.lines().skip(1) {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 3 {
                if let Ok(kb_read) = fields[fields.len() - 2].parse::<f64>() { // KB/s read
                    total_read += (kb_read * 1024.0) as u64; // Convert to bytes
                }
                if let Ok(kb_write) = fields[fields.len() - 1].parse::<f64>() { // KB/s write
                    total_write += (kb_write * 1024.0) as u64; // Convert to bytes
                }
            }
        }

        Ok((total_read, total_write))
    }

    pub fn get_metrics(&mut self) -> WithError<DiskMetrics> {
        let disks = Disks::new_with_refreshed_list();
        let disk = disks.list().first().ok_or("No disk found")?;

        let total_space = disk.total_space();
        let free_space = disk.available_space();
        let used_space = total_space - free_space;

        Ok(DiskMetrics {
            total_space,
            used_space,
            free_space,
        })
    }
}