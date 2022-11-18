use std::env;
use std::process::Command;
use sysinfo::{System, SystemExt};

#[derive(Debug)]
pub struct Sysinfo {
    pub hostname: String,
    pub platform: String,
    pub os_number: String,
    pub cpu: String,
    pub gpu: String,
    pub ram: u64,
    pub mainboard: String,
}

pub fn specs() -> Sysinfo {
    let hostname_output = Command::new("cmd")
        .args(["/C", "wmic path win32_ComputerSystem get name"])
        .output()
        .expect("failed to execute HOSTNAME process");

    let hostname = String::from_utf8(hostname_output.stdout).unwrap();

    let cpu_output = Command::new("cmd")
        .args(["/C", "wmic path win32_Processor get name"])
        .output()
        .expect("failed to execute CPU process");

    let cpu = String::from_utf8(cpu_output.stdout).unwrap();

    let gpu_output = Command::new("cmd")
        .args(["/C", "wmic path win32_VideoController get name"])
        .output()
        .expect("failed to execute GPU process");

    let gpu = String::from_utf8(gpu_output.stdout).unwrap();

    let mainboard_output = Command::new("cmd")
        .args(["/C", "wmic path win32_BaseBoard get Product"])
        .output()
        .expect("failed to execute MAINBOARD process");

    let mainboard = String::from_utf8(mainboard_output.stdout).unwrap();

    let info = os_info::get();
    let mut sys = System::new_all();
    sys.refresh_all();

    // We display all disks' information:

    println!("{:?}", sys.disks()[0]);
    let specs = Sysinfo {
        hostname: String::from(remove_characters(
            hostname.trim().replace("Name             ", ""),
        )),
        platform: String::from(env::consts::OS),
        os_number: info.version().to_string(),
        cpu: String::from(remove_characters(
            cpu.trim()
                .replace("Name                                       ", ""),
        )),
        gpu: String::from(remove_characters(
            gpu.trim().replace("Name                           ", ""),
        )),
        ram: sys.total_memory() / 1024 / 1024,
        mainboard: String::from(remove_characters(
            mainboard
                .trim()
                .replace("Product                        ", ""),
        )),
    };

    return specs;
}

fn remove_characters(data: String) -> String {
    let remove_slash_r = data.replace("\r", "");
    let remove_slash_n = remove_slash_r.replace("\n", "");

    return remove_slash_n;
}
