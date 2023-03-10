use std::{process::Command, fs};

use colored::Colorize;

fn main() {
    let whoami_c = Command::new("whoami").output().expect("Couldn't launch command uname -r");
    let whoami = String::from_utf8_lossy(&whoami_c.stdout);

    let hostname = match fs::read_to_string("/etc/hostname") {
        Ok(hostname) => {hostname},
        Err(_) => {"".to_string()},
    };

    let machinename = match fs::read_to_string("/sys/devices/virtual/dmi/id/product_family") {
        Ok(machinename) => {machinename},
        Err(_) => {"".to_string()},
    };

    let vendorname = match fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor") {
        Ok(vendorname) => {vendorname},
        Err(_) => {"".to_string()},
    };

    let unamer_c = Command::new("uname").arg("-r").output().expect("Couldn't launch command uname -r");
    let kernelversion = String::from_utf8_lossy(&unamer_c.stdout);

    let uptime_content = match fs::read_to_string("/proc/uptime") {
        Ok(uptime_content) => {uptime_content},
        Err(_) => {"".to_string()},
    };
    let uptime_content_splitted: Vec<&str> = uptime_content.split(" ").collect();
    let uptime_totalsecs_predot: Vec<&str> = uptime_content_splitted[0].split(".").collect();

    let uptime_totalsecs: usize = uptime_totalsecs_predot[0].parse().expect("Couldn't parse uptime, how?");

    let uptimed = uptime_totalsecs/(60*60*24);
    let uptimeh = (uptime_totalsecs%(60*60*24))/3600;
    let uptimem = (uptime_totalsecs%(60*60))/60;

    let osrelease_content = match fs::read_to_string("/etc/os-release") {
        Ok(osrelease_content) => {osrelease_content},
        Err(_) => {"".to_string()},
    };
    let osrelease_content_splitted: Vec<&str> = osrelease_content.split("\n").collect();
    let osname_splitted: Vec<&str> = osrelease_content_splitted[0].split("\"").collect();
    let osname = osname_splitted[1];

    let meminfo_content = match fs::read_to_string("/proc/meminfo") {
        Ok(meminfo_content) => {meminfo_content},
        Err(_) => {"".to_string()},
    };
    let meminfo_content_splitted: Vec<&str> = meminfo_content.split("\n").collect();
    
    let totalmem_splitted: Vec<&str> = meminfo_content_splitted[0].split(" ").collect();
    let totalmem: usize = totalmem_splitted[totalmem_splitted.len()-2].parse().expect("Couldn't parse totalmem, how?");

    let freemem_splitted: Vec<&str> = meminfo_content_splitted[1].split(" ").collect();
    let freemem: usize = freemem_splitted[freemem_splitted.len()-2].parse().expect("Couldn't parse freemem, how?");

    let availablemem_splitted: Vec<&str> = meminfo_content_splitted[2].split(" ").collect();
    let availablemem: usize = availablemem_splitted[availablemem_splitted.len()-2].parse().expect("Couldn't parse availablemem, how?");
    
    let usedmemg: f32 = (totalmem-(freemem+availablemem)) as f32/1024 as f32;
    let usedmemg = (usedmemg/102.4).ceil().trunc()/10.0;
    let totalmemg: f32 = totalmem as f32 / 1024 as f32;
    let totalmemg = (totalmemg/102.4).ceil().trunc()/10.0;


    println!("{}{}{}", whoami.trim().to_string().red(), "@".to_string().white(), hostname.trim().to_string().green());

    println!("{label:<8} {osname:<16}", label="os".to_string().blue(), osname=osname);

    println!("{label:<8} {vendorname} {machinename}", label="host".to_string().blue(), vendorname=vendorname.trim(), machinename=machinename.trim());

    println!("{label:<8} {kernelver:<16}", label="kernel".to_string().blue(), kernelver=kernelversion.trim());

    println!("{label:<8} {usedmemg}G / {totalmemg}G", label="mem".to_string().blue(), usedmemg=usedmemg, totalmemg=totalmemg);

    let mut uptimestring: String = String::from("");

    if uptimed != 0 {
        uptimestring += format!("{}d ", uptimed).as_str();
    }
    if uptimeh != 0 {
        uptimestring += format!("{}h ", uptimeh).as_str();
    }
    if uptimem != 0 {
        uptimestring += format!("{}m ", uptimem).as_str();
    }
    uptimestring += format!("{}s ", uptime_totalsecs%60).as_str();
    

    println!("{label:<8} {uptimestring}", label="uptime".to_string().blue(), uptimestring=uptimestring);

}
