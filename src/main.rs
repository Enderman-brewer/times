use chrono::Utc;
use chrono::prelude::*;
use std::env; // 1. Import the env module
use std::os::unix::process::CommandExt; // Required for .exec()
use std::process::Command;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MyFlags: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const E = 1 << 4;
        const F = 1 << 5;
        const G = 1 << 6;
    }
}

fn main() {
    let now = Utc::now();
    let localnow: DateTime<Local> = Local::now();
    let now32 = now.timestamp() as i32; // 32-bit timestamp

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
                println!("  -h, --help    Show this help message");
                println!();
                println!("Exit codes:");
                println!("  0    Success");
                println!("  1    Unknown flag");
                println!("  2    No flags were passed");
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
        println!("{}", now32);
        std::process::exit(0);
    }
    if ab.contains(MyFlags::B) {
        println!("{}", now.timestamp());
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
    if ab.is_empty() {
        println!("No flags were passed.");
        std::process::exit(2);
    }
}
