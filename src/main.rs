use chrono::Utc;
use chrono::prelude::*;
use std::env; // 1. Import the env module

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MyFlags: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const Z = 1 << 4;
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
            "--suppress-text" => ab |= MyFlags::Z,
            "-h" | "--help" => {
                println!("Usage: my_program [options]");
                println!("Options:");
                println!("  -u    Show Unix time (32-bit)");
                println!("  -U    Show Unix time (64-bit)");
                println!("  -S    Show UTC time");
                println!("  -L    Show Local time");
                println!("  -h, --help    Show this help message");
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
        if !ab.contains(MyFlags::Z) {
            println!("{}", now32);
        } else {
            println!("Unix time (32-bit): {}", now32);
        }
    }
    if ab.contains(MyFlags::B) {
        if !ab.contains(MyFlags::Z) {
            println!("{}", now.timestamp());
        } else {
            println!("Unix time (64-bit): {}", now.timestamp());
        }
    }
    if ab.contains(MyFlags::C) {
        if !ab.contains(MyFlags::Z) {
            println!("{}", now);
        } else {
            println!("UTC time: {}", now);
        }
    }
    if ab.contains(MyFlags::D) {
        if !ab.contains(MyFlags::Z) {
            println!("{}", localnow);
        } else {
            println!("Local time: {}", localnow);
        }
    }
    if ab.is_empty() {
        println!("No flags were passed.");
    }
}
