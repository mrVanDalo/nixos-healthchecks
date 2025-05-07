use clap::Parser;
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use env_logger;
use log::debug;
use std::io::{self, stdout, Write};
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

    /// measure script execution and show it
    #[arg(long, default_value_t = false)]
    time: bool,

    /// title to be printed
    #[arg(short, long)]
    title: Option<String>,

    /// The path to the script
    path: String,
}

/// PrettyPrinter
/// containing all the information needed to print user-friendly output.
struct PrettyPrinter {
    /// title of the execution
    title: String,

    /// use emojis in printout or not
    use_emoji: bool,

    /// show execution time or now
    show_time: bool,
}

impl PrettyPrinter {
    fn new(title: String, use_emoji: bool, show_time: bool) -> Self {
        Self {
            title,
            use_emoji,
            show_time,
        }
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
        let mut stdout = stdout();
        // Move to the beginning of the line and clear it
        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

        if self.show_time {
            println!("{} {} [{:.2?}s]", ok, self.title, duration.as_secs_f64());
        } else {
            println!("{} {}", ok, self.title);
        }
    }

    /// print failure line
    fn failure(&self, duration: Duration) {
        let (_, _, fail) = self.exit_code_visualization(self.use_emoji);
        let mut stdout = stdout();
        // Move to the beginning of the line and clear it
        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

        if self.show_time {
            println!("{} {} [{:.2?}s]", fail, self.title, duration.as_secs_f64());
        } else {
            println!("{} {}", fail, self.title);
        }
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
        exit(0)
    } else {
        pretty_printer.failure(duration);
        println!("Output:\n{}", String::from_utf8_lossy(&result.stdout));
        println!("Error:\n{}", String::from_utf8_lossy(&result.stderr));
        exit(1)
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
    debug!("Time measurement is enabled : {}", args.time);

    let pretty_printer = PrettyPrinter::new(
        args.title.unwrap_or(args.path.clone()),
        args.emoji,
        args.time,
    );

    run_script(args.path.as_str(), pretty_printer);
}
