use sysinfo::{
    Components, Disks, Networks, System,
};
use std::env;
use std::fs;
use std::process::Command;

pub struct Info {
    pub os: String,
    pub kernel: String,
    pub host_name: String,
    pub shell: String,
    pub terminal: String,
    pub cpu: String,
    pub gpu: Vec<String>,
    pub memory: String,
    pub swap: String,
    pub disks: Vec<String>,
    pub battery: String,
    pub uptime: String,
    pub packages: String,
    pub desktop: String,
}

impl Info {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let _networks = Networks::new_with_refreshed_list();
        let components = Components::new_with_refreshed_list();

        Self {
            os: get_os_info(),
            kernel: get_kernel_info(),
            host_name: get_host_name(),
            shell: get_shell_info(),
            terminal: get_terminal_info(),
            cpu: get_cpu_info(&sys),
            gpu: get_gpu_info(),
            memory: get_memory_info(&sys),
            swap: get_swap_info(&sys),
            disks: get_disk_info(&disks),
            battery: get_battery_info(&components),
            uptime: get_uptime_info(),
            packages: get_packages_info(),
            desktop: get_desktop_info(),
        }
    }
}

fn get_os_info() -> String {
    let name = System::name().unwrap_or("Unknown".to_string());
    let version = System::os_version().unwrap_or("".to_string());
    let arch = std::env::consts::ARCH;
    format!("{} {} {}", name, version, arch)
}

fn get_kernel_info() -> String {
    let version = System::kernel_version().unwrap_or("Unknown".to_string());
    // On Windows it might be different, but sysinfo handles it usually
    format!("{}", version)
}

fn get_host_name() -> String {
    System::host_name().unwrap_or("Unknown".to_string())
}

fn get_shell_info() -> String {
    // Simple env var check
    if let Ok(shell) = env::var("SHELL") {
        let path = std::path::Path::new(&shell);
        if let Some(name) = path.file_name() {
            return name.to_string_lossy().into_owned();
        }
    }
    // Windows fallback
    if cfg!(target_os = "windows") {
        if env::var("PSModulePath").is_ok() {
            return "PowerShell".to_string();
        }
        return "cmd".to_string();
    }
    "Unknown".to_string()
}

fn get_terminal_info() -> String {
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return term;
    }
    if let Ok(_) = env::var("WT_SESSION") {
        return "Windows Terminal".to_string();
    }
    if let Ok(term) = env::var("TERM") {
        return term;
    }
    "Unknown".to_string()
}

fn get_cpu_info(sys: &System) -> String {
    let cpus = sys.cpus();
    if cpus.is_empty() {
        return "Unknown".to_string();
    }
    let brand = cpus[0].brand();
    let freq = cpus[0].frequency(); // MHz
    let cores = cpus.len();
    
    // Format: Brand (Cores) @ Freq
    format!("{} ({}) @ {:.2} GHz", brand, cores, freq as f64 / 1000.0)
}

fn get_gpu_info() -> Vec<String> {
    let mut gpus = Vec::new();
    
    // Quick hack for Linux
    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new("lspci").arg("-mm").output() {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines() {
                if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                     // Parse lspci -mm output is roughly: "Slot" "Class" "Vendor" "Device"
                     // We just want a rough string for now
                     let parts: Vec<&str> = line.split('"').collect();
                     if parts.len() > 5 {
                         gpus.push(parts[5].to_string());
                     }
                }
            }
        }
    }
    // Windows
    else if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("wmic").args(&["path", "win32_videocontroller", "get", "name"]).output() {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines().skip(1) { // Skip header
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    gpus.push(trimmed.to_string());
                }
            }
        }
    }
    // MacOS
    else if cfg!(target_os = "macos") {
        if let Ok(output) = Command::new("system_profiler").arg("SPDisplaysDataType").output() {
             let out = String::from_utf8_lossy(&output.stdout);
             for line in out.lines() {
                 if line.trim().starts_with("Chipset Model:") {
                     gpus.push(line.trim().replace("Chipset Model: ", ""));
                 }
             }
        }
    }

    if gpus.is_empty() {
        gpus.push("Unknown GPU".to_string());
    }
    gpus
}

fn get_memory_info(sys: &System) -> String {
    let total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0; // GiB
    let used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let percent = (used / total) * 100.0;
    format!("{:.2} GiB / {:.2} GiB ({:.0}%)", used, total, percent)
}

fn get_swap_info(sys: &System) -> String {
    let total = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    if total == 0.0 {
        return "0 B / 0 B (0%)".to_string();
    }
    let percent = (used / total) * 100.0;
    format!("{:.2} GiB / {:.2} GiB ({:.0}%)", used, total, percent)
}

fn get_disk_info(disks: &Disks) -> Vec<String> {
    let mut disk_list = Vec::new();
    for disk in disks {
        let total = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used = total - available;
        let percent = (used / total) * 100.0;
        let fs = disk.file_system().to_str().unwrap_or("Unknown");
        // Windows often has many disks, maybe filter by fixed?
        // sysinfo disks usually includes fixed.
        disk_list.push(format!("{:.2} GiB / {:.2} GiB ({:.0}%) - {}", used, total, percent, fs));
    }
    disk_list
}

fn get_battery_info(_components: &Components) -> String {
    // sysinfo Components sometimes has battery, but usually better to use a crate or /sys/class/power_supply
    // For now, simple placeholder or check /sys/class
    #[cfg(target_os = "linux")]
    {
        // Simple check
        if let Ok(cap) = fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
            let status = fs::read_to_string("/sys/class/power_supply/BAT0/status").unwrap_or("Unknown".to_string());
            return format!("{}% [{}]", cap.trim(), status.trim());
        }
    }
    "100% [AC Connected]".to_string() // Fallback
}

fn get_uptime_info() -> String {
    let uptime = System::uptime();
    let hours = uptime / 3600;
    let mins = (uptime % 3600) / 60;
    format!("{} hours, {} mins", hours, mins)
}

fn get_packages_info() -> String {
    // Count packages
    // Linux: check pacman, dpkg, etc.
    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new("pacman").arg("-Qq").output() {
            if output.status.success() {
                 let count = String::from_utf8_lossy(&output.stdout).lines().count();
                 return format!("{} (pacman)", count);
            }
        }
        if let Ok(output) = Command::new("dpkg").arg("--get-selections").output() {
             if output.status.success() {
                 let count = String::from_utf8_lossy(&output.stdout).lines().count();
                 return format!("{} (dpkg)", count);
             }
        }
    }
    // Windows
    if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("scoop").arg("list").output() {
             if output.status.success() {
                 let count = String::from_utf8_lossy(&output.stdout).lines().count().saturating_sub(4); // scoop header
                 return format!("{} (scoop)", count);
             }
        }
    }
    "Unknown".to_string()
}

fn get_desktop_info() -> String {
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        return de;
    }
    if let Ok(de) = env::var("DESKTOP_SESSION") {
        return de;
    }
    if cfg!(target_os = "windows") {
        return "Explorer".to_string();
    }
    if cfg!(target_os = "macos") {
        return "Aqua".to_string();
    }
    "Unknown".to_string()
}
