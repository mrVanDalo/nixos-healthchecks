use std::path::Path;
use std::process::{exit, Command};

use clap::Parser;
use env_logger;
use log::debug;

/// Run healthcheck commands and make them look pretty.
#[derive(Parser, Debug)]
#[command(
    author = "Ingolf Wagner <contact@ingolf-wagner.de>",
    version = "1.0",
    about = "print out healthcheck script lines"
)]
struct Args {
    /// use emojis to print response code
    #[arg(long, default_value_t = false)]
    emoji: bool,

    /// title to be printed
    #[arg(short, long)]
    title: Option<String>,

    /// The path to the script
    path: String,
}

fn exit_code_visualization(use_emoji: bool) -> (&'static str, &'static str) {
    if use_emoji {
        ("✅", "❌")
    } else {
        ("[ OK ]", "[Fail]")
    }
}

fn run_script(script: &str, title: &str, use_emoji: bool) {
    let (ok, fail) = exit_code_visualization(use_emoji);

    if !Path::new(script).exists() {
        println!("{} {}", fail, title);
        eprintln!("{} does not exist", script);
        exit(1);
    }

    let result = Command::new(script)
        .output()
        .expect("Failed to execute script");

    if result.status.success() {
        println!("{} {}", ok, title);
    } else {
        println!("{} {}", fail, title);
        println!("Output:\n{}", String::from_utf8_lossy(&result.stdout));
        println!("Error:\n{}", String::from_utf8_lossy(&result.stderr));
    }
}

fn main() {
    // Initialize the logger
    env_logger::init();

    // Parse command line arguments
    let args = Args::parse();

    debug!("Title: {:?}", args.title);
    debug!("Script path: {}", args.path);
    debug!("Emojis enabled : {}", args.emoji);

    run_script(
        args.path.clone().as_str(),
        args.title.unwrap_or(args.path).as_str(),
        args.emoji,
    );
}
