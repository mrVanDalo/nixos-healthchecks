use clap::Parser;
use env_logger;
use log::debug;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::time::Instant;

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

fn exit_code_visualization(use_emoji: bool) -> (&'static str, &'static str, &'static str) {
    if use_emoji {
        ("⏳", "✅", "❌")
    } else {
        ("[Wait]", "[ OK ]", "[Fail]")
    }
}

fn run_script(script: &str, title: &str, use_emoji: bool) {
    let (wait, ok, fail) = exit_code_visualization(use_emoji);

    if !Path::new(script).exists() {
        println!("{} {}", fail, title);
        eprintln!("{} does not exist", script);
        exit(1);
    }

    print!("{} {}", wait, title);
    io::stdout().flush().unwrap();

    let start = Instant::now();
    let result = Command::new(script)
        .output()
        .expect("Failed to execute script");
    let duration = start.elapsed();
    print!("\r\x1B[2K"); // \x1B[2K clears the entire line

    if result.status.success() {
        println!("{} {} [{:.2?}s]", ok, title, duration.as_secs_f64());
    } else {
        println!("{} {} [{:.2?}s]", fail, title, duration.as_secs_f64());
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
