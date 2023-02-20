use std::{process::Command, fs};

use colored::Colorize;

fn main() {
    let whoami_c = Command::new("whoami").output().expect("Couldn't launch command whoami");
    let whoami = String::from_utf8_lossy(&whoami_c.stdout);

    /*
    let hostname_c = Command::new("hostname").output().expect("Couldn't launch command hostnamectl");
    let hostname = String::from_utf8_lossy(&hostname_c.stdout);
    */

    let hostname = fs::read_to_string("/etc/hostname").expect("Couldn't read /etc/hostname");

    // cat /sys/devices/virtual/dmi/id/product_family


    let machinename = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").expect("Couldn't read /sys/devices/virtual/dmi/id/product_family");

    let vendorname = fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor").expect("Couldn't read /sys/devices/virtual/dmi/id/sys_vendor");

    let unamer_c = Command::new("uname").arg("-r").output().expect("Couldn't launch command uname -r");
    let kernelversion = String::from_utf8_lossy(&unamer_c.stdout);
    //println!("meminfo output: {}", String::from_utf8_lossy(&meminfoc.stdout));

    let uptime_content = fs::read_to_string("/proc/uptime").expect("Couldn't read /proc/uptime");
    let uptime_content_splitted: Vec<&str> = uptime_content.split(" ").collect();
    let uptime_totalsecs_predot: Vec<&str> = uptime_content_splitted[0].split(".").collect();

    let uptime_totalsecs: usize = uptime_totalsecs_predot[0].parse().expect("Couldn't parse uptime, how?");

    let uptimeh = uptime_totalsecs/3600;
    let uptimem = (uptime_totalsecs%3600)/60;

    let osrelease_content = fs::read_to_string("/etc/os-release").expect("Couldn't read /etc/os-release");
    let osrelease_content_splitted: Vec<&str> = osrelease_content.split("\n").collect();
    let osname_splitted: Vec<&str> = osrelease_content_splitted[0].split("\"").collect();
    let osname = osname_splitted[1];

    let meminfo_content = fs::read_to_string("/proc/meminfo").expect("Couldn't read /proc/meminfo");
    let meminfo_content_splitted: Vec<&str> = meminfo_content.split("\n").collect();
    
    let totalmem_splitted: Vec<&str> = meminfo_content_splitted[0].split(" ").collect();
    let totalmem: usize = totalmem_splitted[totalmem_splitted.len()-2].parse().expect("Couldn't parse totalmem, how?");

    let freemem_splitted: Vec<&str> = meminfo_content_splitted[1].split(" ").collect();
    let freemem: usize = freemem_splitted[freemem_splitted.len()-2].parse().expect("Couldn't parse freemem, how?");

    let availablemem_splitted: Vec<&str> = meminfo_content_splitted[2].split(" ").collect();
    let availablemem: usize = availablemem_splitted[availablemem_splitted.len()-2].parse().expect("Couldn't parse availablemem, how?");
    
    let usedmemg: f32 = (totalmem-(freemem+availablemem)) as f32/1024 as f32;
    let usedmemg = (usedmemg/100.0).trunc()/10.0;
    let totalmemg: f32 = totalmem as f32 / 1024 as f32;
    let totalmemg = (totalmemg/100.0).trunc()/10.0;


    println!("{}{}{}", whoami.trim().to_string().red(), "@".to_string().white(), hostname.trim().to_string().green());

    println!("{} {osname:>16}","os".to_string().blue(), osname=osname);

    println!("{} {vendorname:>8} {machinename:>8}", "host".to_string().blue(), vendorname=vendorname.trim(), machinename=machinename.trim());

    println!("{} {kernelver:>8}", "kernel".to_string().blue(), kernelver=kernelversion.trim());

    println!("{} {usedmemg:>7}G / {totalmemg}G", "mem".to_string().blue(), usedmemg=usedmemg, totalmemg=totalmemg);

    println!("{} {}h {}m {}s", "uptime".to_string().blue(), uptimeh, uptimem, uptime_totalsecs%60);



}
