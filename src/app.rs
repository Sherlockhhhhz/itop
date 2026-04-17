use std::{
    collections::VecDeque,
    time::Instant,
};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::gpu::GpuStats;

pub const HISTORY_LEN: usize = 60;

pub struct App {
    pub sys: System,
    pub cpu_history: VecDeque<(f64, f64)>,
    pub mem_history: VecDeque<(f64, f64)>,
    pub gpu_history: VecDeque<(f64, f64)>,
    pub gpu: Option<GpuStats>,
    pub tick: u64,
    pub last_update: Instant,
}

impl App {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        App {
            sys,
            cpu_history: VecDeque::with_capacity(HISTORY_LEN),
            mem_history: VecDeque::with_capacity(HISTORY_LEN),
            gpu_history: VecDeque::with_capacity(HISTORY_LEN),
            gpu: None,
            tick: 0,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        let t = self.tick as f64;
        let cpu_pct = self.sys.global_cpu_usage() as f64;
        let mem_total = self.sys.total_memory() as f64;
        let mem_pct = if mem_total == 0.0 { 0.0 } else { self.sys.used_memory() as f64 / mem_total * 100.0 };

        push_bounded(&mut self.cpu_history, (t, cpu_pct), HISTORY_LEN);
        push_bounded(&mut self.mem_history, (t, mem_pct), HISTORY_LEN);

        // GPU via ioreg (Apple Silicon only)
        self.gpu = GpuStats::query();
        let gpu_pct = self.gpu.as_ref().map(|g| g.utilization_pct).unwrap_or(0.0);
        push_bounded(&mut self.gpu_history, (t, gpu_pct), HISTORY_LEN);

        self.tick += 1;
        self.last_update = Instant::now();
    }

    pub fn cpu_usage(&self) -> f64 {
        self.sys.global_cpu_usage() as f64
    }

    pub fn mem_used_gb(&self) -> f64 {
        self.sys.used_memory() as f64 / 1_073_741_824.0
    }

    pub fn mem_total_gb(&self) -> f64 {
        self.sys.total_memory() as f64 / 1_073_741_824.0
    }

    pub fn mem_pct(&self) -> f64 {
        let total = self.sys.total_memory() as f64;
        if total == 0.0 { 0.0 } else { self.sys.used_memory() as f64 / total * 100.0 }
    }

    pub fn swap_pct(&self) -> f64 {
        let total = self.sys.total_swap() as f64;
        if total == 0.0 { 0.0 } else { self.sys.used_swap() as f64 / total * 100.0 }
    }

    pub fn swap_used_gb(&self) -> f64 {
        self.sys.used_swap() as f64 / 1_073_741_824.0
    }

    pub fn swap_total_gb(&self) -> f64 {
        self.sys.total_swap() as f64 / 1_073_741_824.0
    }

    pub fn per_cpu(&self) -> Vec<f64> {
        self.sys.cpus().iter().map(|c| c.cpu_usage() as f64).collect()
    }
}

fn push_bounded(dq: &mut VecDeque<(f64, f64)>, val: (f64, f64), cap: usize) {
    dq.push_back(val);
    if dq.len() > cap {
        dq.pop_front();
    }
}
