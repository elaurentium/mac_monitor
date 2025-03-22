use serde::Serialize;
use std::time::Instant;
use std::process::Command;
use std::io;

type WithError<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default, Serialize)]
pub struct NetworkMetrics {
    pub download_bytes: u64,
    pub upload_bytes: u64,
    pub total_down: u64,
    pub total_up: u64,
}

#[allow(dead_code)]
pub struct NetworkSampler {
    last_down: u64,
    last_up: u64,
    total_down: u64,
    total_up: u64,
    last_time: Instant,
}

impl NetworkSampler {
    pub fn new() -> WithError<Self> {
        Ok(NetworkSampler {
            last_down: 0,
            last_up: 0,
            total_down: 0,
            total_up: 0,
            last_time: Instant::now(),
        })
    }

    fn get_network_stats(&self) -> WithError<(u64, u64)> {
        let output = Command::new("netstat")
            .arg("-ib") // Interface bytes
            .output()?;

        if !output.status.success() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "netstat command failed",
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut total_down = 0;
        let mut total_up = 0;

        // Parse netstat -ib output
        for line in stdout.lines().skip(1) { // Skip header
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 7 {
                // Fields: Name, Mtu, Network, Address, Ipkts, Ibytes, Opkts, Obytes
                if let Ok(ibytes) = fields[5].parse::<u64>() { // Input bytes
                    total_down += ibytes;
                }
                if let Ok(obytes) = fields[7].parse::<u64>() { // Output bytes
                    total_up += obytes;
                }
            }
        }

        Ok((total_down, total_up))
    }

    pub fn get_metrics(&mut self) -> WithError<NetworkMetrics> {
        let (current_down, current_up) = self.get_network_stats()?;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_time).as_secs_f64();

        self.total_down = self.total_down.saturating_add(current_down);
        self.total_up = self.total_up.saturating_add(current_up);

        let download_bytes = if elapsed > 0.0 {
            ((current_down.saturating_sub(self.last_down)) as f64 / elapsed) as u64
        } else {
            0
        };

        let upload_bytes = if elapsed > 0.0 {
            ((current_up.saturating_sub(self.last_up)) as f64 / elapsed) as u64
        } else {
            0
        };

        self.last_down = current_down;
        self.last_up = current_up;
        self.last_time = now;

        Ok(NetworkMetrics {
            download_bytes,
            upload_bytes,
            total_down: self.total_down,
            total_up: self.total_up,
        })
    }
}