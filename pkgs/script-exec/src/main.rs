use clap::Parser;
use env_logger;
use log::debug;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::time::{Duration, Instant};

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

/// PrettyPrinter
/// containing all the information needed to print user-friendly output.
struct PrettyPrinter {
    /// use emojis in printout or not
    use_emoji: bool,

    /// title of the execution
    title: String,
}

impl PrettyPrinter {
    fn new(title: String, use_emoji: bool) -> Self {
        Self { use_emoji, title }
    }

    /// decision function on what to print
    fn exit_code_visualization(
        &self,
        use_emoji: bool,
    ) -> (&'static str, &'static str, &'static str) {
        if use_emoji {
            ("⏳", "✅", "❌")
        } else {
            ("[Wait]", "[ OK ]", "[Fail]")
        }
    }

    /// print waiting line
    fn waiting(&self) {
        let (wait, _, _) = self.exit_code_visualization(self.use_emoji);
        print!("{} {}", wait, self.title);
        io::stdout().flush().unwrap();
    }

    /// print success line
    fn success(&self, duration: Duration) {
        let (_, ok, _) = self.exit_code_visualization(self.use_emoji);
        print!("\r\x1B[2K"); // \x1B[2K clears the entire line
        println!("{} {} [{:.2?}s]", ok, self.title, duration.as_secs_f64());
    }

    /// print failure line
    fn failure(&self, duration: Duration) {
        let (_, _, fail) = self.exit_code_visualization(self.use_emoji);
        print!("\r\x1B[2K"); // \x1B[2K clears the entire line
        println!("{} {} [{:.2?}s]", fail, self.title, duration.as_secs_f64());
    }
}

fn run_script(script: &str, pretty_printer: PrettyPrinter) {
    if !Path::new(script).exists() {
        pretty_printer.failure(Duration::new(0, 0));
        eprintln!("{} does not exist", script);
        exit(1);
    }

    pretty_printer.waiting();

    let start = Instant::now();
    let result = Command::new(script)
        .output()
        .expect("Failed to execute script");
    let duration = start.elapsed();

    if result.status.success() {
        pretty_printer.success(duration);
    } else {
        pretty_printer.failure(duration);
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

    let pretty_printer = PrettyPrinter::new(args.title.unwrap_or(args.path.clone()), args.emoji);

    run_script(args.path.as_str(), pretty_printer);
}
