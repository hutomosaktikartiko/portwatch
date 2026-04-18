use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sysinfo::{Networks, Pid, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

trait PortScanner {
    fn scan(&mut self, min_port: u16) -> Result<Vec<PortInfo>>;
}

#[cfg(target_os = "macos")]
struct MacScanner {
    system: System,
}

#[cfg(target_os = "linux")]
struct LinuxScanner;

#[cfg(target_os = "windows")]
struct WindowsScanner;

#[cfg(target_os = "macos")]
impl MacScanner {
    pub fn new() -> Self {
        MacScanner {
            system: System::new_all(),
        }
    }

    fn enrich(&self, mut info: PortInfo) -> PortInfo {
        use std::process::Command;

        let output = Command::new("lsof")
            .args(["-i", &format!(":{}", info.port), "-P", "-n", "-sTCP:LISTEN"])
            .output();

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines().skip(1) {
                let cols: Vec<&str> = line.split_whitespace().collect();
                if cols.len() >= 2 {
                    info.process_name = Some(cols[0].to_string());
                    info.pid = cols[1].parse::<u32>().ok();
                    break;
                }
            }
        }
        info
    }
}

#[cfg(target_os = "macos")]
impl PortScanner for MacScanner {
    fn scan(&mut self, min_port: u16) -> Result<Vec<PortInfo>> {
        use std::process::Command;

        let output = Command::new("netstat")
            .args(["-anp", "tcp"])
            .output()
            .context("Failed to execute netstat")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut ports = Vec::new();

        for line in stdout.lines() {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() < 6 {
                continue;
            }

            let proto = cols[0];
            let state = cols[5];

            if !proto.starts_with("tcp") {
                continue;
            }
            if state != "LISTEN" {
                continue;
            }

            // Format address
            if let Some(port_str) = cols[3].rsplit('.').next() {
                if let Ok(port) = port_str.parse::<u16>() {
                    if port >= min_port {
                        let raw = PortInfo {
                            port,
                            protocol: "TCP".to_string(),
                            pid: None,
                            process_name: None,
                        };
                        ports.push(self.enrich(raw));
                    }
                }
            }
        }

        ports.sort_by_key(|p| p.port);
        ports.dedup_by_key(|p| p.port);
        Ok(ports)
    }
}

#[cfg(target_os = "linux")]
impl PortScanner for LinuxScanner {
    fn scan(&mut self, min_port: u16) -> Result<Vec<PortInfo>> {
        anyhow::bail!("Linux support comming soon!");
    }
}

#[cfg(target_os = "windows")]
impl PortScanner for WindowsScanner {
    fn scan(&mut self, min_port: u16) -> Result<Vec<PortInfo>> {
        anyhow::bail!("Windows support comming soon!");
    }
}

pub struct Scanner {
    #[cfg(target_os = "macos")]
    inner: MacScanner,
    #[cfg(target_os = "linux")]
    inner: LinuxScanner,
    #[cfg(target_os = "windows")]
    inner: WindowsScanner,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            #[cfg(target_os = "macos")]
            inner: MacScanner::new(),
            #[cfg(target_os = "linux")]
            inner: LinuxScanner,
            #[cfg(target_os = "windows")]
            inner: WindowsScanner,
        }
    }

    /// Scan and return open ports
    pub fn scan(&mut self, min_port: u16) -> Result<Vec<PortInfo>> {
        self.inner.scan(min_port)
    }
}
