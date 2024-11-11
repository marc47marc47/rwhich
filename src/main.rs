use std::env;

fn main() {
    // Get command-line arguments, skipping the program name
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        eprintln!("Usage: which-rs [options] command...");
        eprintln!("       rwhich ls");
        eprintln!("       rwhich openssl");
        eprintln!("       rwhich opewnssl.exe");
        eprintln!("       rwhich code.exe");
        eprintln!("");
        eprintln!("Options:");
        eprintln!("  -h, --help    Display this help message");
        std::process::exit(1);
    }

    // Call the which function from lib.rs
    rwhich::which(&args);
}

