use serde::Serialize;
use std::fs;
use std::io;

type WithError<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default, Serialize)]
pub struct DiskMetrics {
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
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
        let stats = fs::metadata("/")?;
        Ok((stats.len(), 0)) // Placeholder
    }

    pub fn get_metrics(&mut self) -> WithError<DiskMetrics> {
        let path = "/";
        let statvfs = unsafe {
            let mut statvfs = std::mem::zeroed::<libc::statvfs>();
            if libc::statvfs(path.as_ptr() as *const i8, &mut statvfs) != 0 {
                return Err(Box::new(io::Error::last_os_error()));
            }
            statvfs
        };

        let block_size = statvfs.f_frsize as u64;
        let total_space = statvfs.f_blocks as u64 * block_size;
        let free_space = statvfs.f_bavail as u64 * block_size;
        let used_space = total_space - free_space;

        let (current_read, current_write) = self.get_disk_io()?;
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_time).as_secs_f64();

        let read_bytes = if elapsed > 0.0 {
            ((current_read.saturating_sub(self.last_read)) as f64 / elapsed) as u64
        } else {
            0
        };
        
        let write_bytes = if elapsed > 0.0 {
            ((current_write.saturating_sub(self.last_write)) as f64 / elapsed) as u64
        } else {
            0
        };

        self.last_read = current_read;
        self.last_write = current_write;
        self.last_time = now;

        Ok(DiskMetrics {
            total_space,
            used_space,
            free_space,
            read_bytes,
            write_bytes,
        })
    }
}