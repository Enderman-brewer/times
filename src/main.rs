use chrono::Utc;
use chrono::prelude::*;
use std::env; // 1. Import the env module
use std::os::unix::process::CommandExt; // Required for .exec()
use std::process::Command;
use base64::{engine::general_purpose, Engine as _};


bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MyFlags: u16 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const E = 1 << 4;
        const F = 1 << 5;
        const G = 1 << 6;
        const H = 1 << 7;
        const I = 1 << 8;
        const J = 1 << 9;
        const K = 1 << 10;
        const L = 1 << 11;
        const M = 1 << 12;
    }
}

fn print_gradient(text: &str, start: (u8, u8, u8), end: (u8, u8, u8)) {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    for (i, &c) in chars.iter().enumerate() {
        // Compute interpolation ratio
        let r = start.0 as f32 + (end.0 as f32 - start.0 as f32) * i as f32 / (len.max(1) - 1).max(1) as f32;
        let g = start.1 as f32 + (end.1 as f32 - start.1 as f32) * i as f32 / (len.max(1) - 1).max(1) as f32;
        let b = start.2 as f32 + (end.2 as f32 - start.2 as f32) * i as f32 / (len.max(1) - 1).max(1) as f32;

        // Print character with truecolor ANSI code
        print!("\x1b[38;2;{};{};{}m{}", r as u8, g as u8, b as u8, c);
    }
    
    // Reset terminal styles at the end
    println!("\x1b[0m");
}

fn main() {
    let now = Utc::now();
    let localnow: DateTime<Local> = Local::now();
    
    // Uncomment to get info on launch time
    // println!("Current UTC time: {}", now);
    // println!("Timestamp (seconds): {}", now.timestamp());
    // println!("You know the time is: {}", localnow);
    
    // 2. Start with an empty set of flags
    let mut ab = MyFlags::empty();
    
    // 3. Read arguments passed from the shell (skipping the binary name itself)
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-u" => ab |= MyFlags::A,
            "-U" => ab |= MyFlags::B,
            "-S" => ab |= MyFlags::C,
            "-L" => ab |= MyFlags::D,
            "-E" => ab |= MyFlags::E,
            "-P" => ab |= MyFlags::F,
            "-F" => ab |= MyFlags::G,
            "-Y" => ab |= MyFlags::H,
            "-B" => ab |= MyFlags::I,
            "-s" => ab |= MyFlags::J,
            "-b" => ab |= MyFlags::K,
            "-T" => ab |= MyFlags::L,
            "-B64" => ab |= MyFlags::M,
            "-h" | "--help" => {
                println!("Usage: times [options]");
                println!("Options:");
                println!("  -u    Show Unix time (32-bit)");
                println!("  -U    Show Unix time (64-bit)");
                println!("  -S    Show UTC time");
                println!("  -L    Show Local time");
                println!("  -E    Show  time");
                println!("  -P    Show pager time");
                println!("  -F    The time, but forever");
                println!("  -Y    Time until Y2K38");
                println!("  -B    Web browser time");
                println!("  -b    Web browser time, but with JS");
                println!("  -s    Attempt to use shaders");
                println!("  -T    Themed");
                println!("  -B64  Base64");
                println!("  -h, --help    Show this help message");
                println!();
                println!("Exit codes:");
                println!("  0    Success");
                println!("  1    Unknown flag");
                println!("  2    No flags were passed");
                println!("  Anything else    Who the fuck knows");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown flag: {}", arg);
                std::process::exit(1);
            }
        }
    }
    
    // 4. Check which flags were actually set by the user
    if ab.contains(MyFlags::A) {
        let now32 = now.timestamp() as i32; // 32-bit timestamp
        println!("{}", now32);
        std::process::exit(0);
    }
    if ab.contains(MyFlags::B) {
        println!("{}", now.timestamp()); // 64-bit timestamp
        std::process::exit(0);
    }
    if ab.contains(MyFlags::C) {
        println!("{}", now);
        std::process::exit(0);
    }
    if ab.contains(MyFlags::D) {
        println!("{}", localnow);
        std::process::exit(0);
    }
    if ab.contains(MyFlags::E) {
        println!("\x1b[31m12:00:00 AM\x1b[0m");
        std::process::exit(0);
    }
    if ab.contains(MyFlags::F) {
        let error = Command::new("bash")
            .arg("-c")
            .arg("date | less")
            .exec(); // This replaces the current process with the new command
        eprintln!("Failed to execute pager: {}", error);
        std::process::exit(1);
    }
    if ab.contains(MyFlags::G) {
        loop {
            println!("{}", Utc::now());
        }
    }
    if ab.contains(MyFlags::H) {
        let y2k38 = Utc.ymd(2038, 1, 19).and_hms(3, 14, 7);
        let now = Utc::now();
        let duration_until_y2k38 = y2k38.signed_duration_since(now);
        println!("Time until Y2K38: {} seconds", duration_until_y2k38.num_seconds());
        std::process::exit(0);
    }
    if ab.contains(MyFlags::I) {
        let html = format!(
            "<html><body><h1>Current Time</h1><p>{}</p></body></html>",
            now
        );
        let temp_file_path = "/tmp/current_time.html";
        std::fs::write(temp_file_path, html).expect("Unable to write to temporary file");
        let error = Command::new("bash")
            .arg("-c")
            .arg(format!("xdg-open {}", temp_file_path))
            .exec(); // This replaces the current process with the new command
        eprintln!("Failed to open web browser: {}", error);
        std::process::exit(1);
    }
  if ab.contains(MyFlags::J) {
        println!("\x1b[95m{}\x1b[0m", localnow);
        println!("Error, shader GLOSSY-2 from \"ENDER'S SHADERS\" failed to load.");
        std::process::exit(0);
  }
    if ab.contains(MyFlags::K) {
        let html = format!(
            r#"<html><body><h1>Current Time</h1><p>Compiled on: {}</p><p id="time"></p><script>setInterval(()=>document.getElementById("time").textContent=new Date().toISOString().replace("T"," ").replace("Z"," UTC"),1000)</script></body></html>"#,
            now
        );
        let temp_file_path = "/tmp/current_time_js.html";
        std::fs::write(temp_file_path, html).expect("Unable to write to temporary file");
        let error = Command::new("bash")
            .arg("-c")
            .arg(format!("xdg-open {}", temp_file_path))
            .exec(); // This replaces the current process with the new command
        eprintln!("Failed to open web browser: {}", error);
        std::process::exit(1);
    }
    if ab.contains(MyFlags::L) {
        let local_str = localnow.format("%Y-%m-%d %H:%M:%S.%f %:z").to_string();
        print_gradient(&local_str, (200, 10, 10), (70, 70, 70));
    }
    if ab.contains(MyFlags::M) {
        let local_str = localnow.format("%Y-%m-%d %H:%M:%S.%f %:z").to_string();
        let encoded_text = general_purpose::STANDARD.encode(local_str.as_bytes());
        println!("{}", encoded_text);


    }
    if ab.is_empty() {
        println!("No flags were passed.");
        std::process::exit(2);
    }
}
