/// Apple Silicon MPS/GPU metrics via `ioreg`.
///
/// Parses `IOAccelerator` entries to extract GPU utilization reported by
/// the Metal Performance Shaders runtime.  No `sudo` required.
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct GpuStats {
    /// 0–100
    pub utilization_pct: f64,
    pub device_name: String,
}

impl GpuStats {
    /// Query GPU stats from ioreg. Returns `None` if unavailable.
    pub fn query() -> Option<Self> {
        let output = Command::new("ioreg")
            .args(["-r", "-d", "1", "-w", "0", "-c", "IOAccelerator"])
            .output()
            .ok()?;

        let text = String::from_utf8_lossy(&output.stdout);
        parse_ioreg(&text)
    }
}

fn parse_ioreg(text: &str) -> Option<GpuStats> {
    let mut util: Option<f64> = None;
    let mut name = String::from("Apple GPU");

    for line in text.lines() {
        let line = line.trim();

        // e.g.  "IOClass" = "AGXAcceleratorG14X"
        if line.contains("\"IOClass\"") {
            if let Some(val) = extract_string_value(line) {
                name = val;
            }
        }

        // e.g.  "PerformanceStatistics" = {"Device Utilization %"=12,...}
        if line.contains("\"PerformanceStatistics\"") {
            if let Some(u) = extract_device_util(line) {
                util = Some(u);
            }
        }
    }

    util.map(|u| GpuStats {
        utilization_pct: u,
        device_name: prettify_class_name(&name),
    })
}

fn extract_string_value(line: &str) -> Option<String> {
    // Format:  "Key" = "Value"
    let eq = line.find('=')?;
    let rhs = line[eq + 1..].trim();
    if rhs.starts_with('"') && rhs.ends_with('"') && rhs.len() >= 2 {
        Some(rhs[1..rhs.len() - 1].to_string())
    } else {
        None
    }
}

fn extract_device_util(line: &str) -> Option<f64> {
    // Look for  "Device Utilization %"=<number>
    let key = "\"Device Utilization %\"=";
    let start = line.find(key)? + key.len();
    let rest = &line[start..];
    let end = rest.find([',', '}', ' ']).unwrap_or(rest.len());
    rest[..end].trim().parse::<f64>().ok()
}

fn prettify_class_name(raw: &str) -> String {
    // "AGXAcceleratorG14X" → "Apple M3 Pro GPU" style shortening
    if raw.contains("AGX") {
        "Apple Silicon GPU (AGX)".to_string()
    } else {
        raw.to_string()
    }
}
