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
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Clone)]
enum OutputCommand {
    AddTask(String),
    CompleteTask {
        title: String,
        success: bool,
        duration: Duration,
        output: Option<String>,
    },
    Error {
        title: String,
        message: String,
    },
}

struct OutputManager {
    sender: Sender<OutputCommand>,
}

impl OutputManager {
    fn new() -> Self {
        let (sender, receiver) = channel();

        // Spawn the output thread
        thread::spawn(move || {
            let mut display_state = DisplayState::new();
            while let Ok(command) = receiver.recv() {
                match command {
                    OutputCommand::AddTask(title) => {
                        display_state.add_task(title);
                    }
                    OutputCommand::CompleteTask {
                        title,
                        success,
                        duration,
                        output,
                    } => {
                        let result_line = if success {
                            format!("✅ {} [{:.2?}s]", title, duration.as_secs_f64())
                        } else {
                            let mut lines =
                                vec![format!("❌ {} [{:.2?}s]", title, duration.as_secs_f64())];
                            if let Some(output) = output {
                                lines.push(output);
                            }
                            lines.join("\n")
                        };
                        display_state.remove_task(&title, result_line);
                    }
                    OutputCommand::Error { title, message } => {
                        display_state.add_error_output(vec![format!("❌ {}", title), message]);
                    }
                }
            }
        });

        Self { sender }
    }

    fn send(&self, command: OutputCommand) {
        self.sender.send(command).unwrap();
    }
}

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

    /// Number of parallel jobs
    #[arg(short = 'j', long = "jobs", default_value_t = 3)]
    jobs: usize,

    /// The paths to the scripts
    paths: Vec<String>,
}

fn main() {
    env_logger::init();
    let mut args = Args::parse();
    args.paths.reverse();

    if args.paths.is_empty() {
        eprintln!("No paths provided");
        exit(1);
    }

    let output_manager = Arc::new(OutputManager::new());

    // Create ScriptContainers before spawning threads
    let script_containers: Vec<ScriptContainer> = args
        .paths
        .into_iter()
        .map(|path| ScriptContainer::new(args.emoji, args.time, path))
        .collect();

    let mut handles = vec![];
    let containers = Arc::new(Mutex::new(script_containers));

    // Spawn worker threads
    for _ in 0..args.jobs {
        let containers_clone = Arc::clone(&containers);
        let output_manager = Arc::clone(&output_manager);

        let handle = thread::spawn(move || loop {
            let container = {
                let mut containers = containers_clone.lock().unwrap();
                if containers.is_empty() {
                    break;
                }
                containers.pop().unwrap()
            };

            run_single_script(container, output_manager.clone());
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Clone)]
struct RunningTask {
    title: String,
    start_time: Instant,
}

struct DisplayState {
    running_tasks: Vec<RunningTask>,
    completed_lines: Vec<String>,
}

impl DisplayState {
    fn new() -> Self {
        Self {
            running_tasks: Vec::new(),
            completed_lines: Vec::new(),
        }
    }

    fn add_task(&mut self, title: String) {
        self.clear_waiting();
        self.running_tasks.push(RunningTask {
            title,
            start_time: Instant::now(),
        });
        self.print_waiting()
    }

    fn remove_task(&mut self, title: &str, result_line: String) {
        self.clear_waiting();
        self.running_tasks.retain(|task| task.title != title);
        println!("{}", result_line);
        self.print_waiting()
    }

    fn clear_waiting(&self) {
        let mut stdout = stdout();

        // Calculate total lines to clear (completed + running tasks)
        //let total_lines = self.completed_lines.len() + self.running_tasks.len();
        let total_lines = self.running_tasks.len();

        // Clear previous output
        for _ in 0..total_lines {
            stdout.execute(cursor::MoveUp(1)).unwrap();
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        }
    }

    fn print_waiting(&self) {
        // Print running tasks
        for task in &self.running_tasks {
            let elapsed = task.start_time.elapsed();
            println!(
                "⏳ {} (Running for {:.1}s)",
                task.title,
                elapsed.as_secs_f64()
            );
        }
    }

    fn add_error_output(&mut self, error_lines: Vec<String>) {
        self.clear_waiting();
        for line in error_lines {
            println!("{}", line);
        }
        self.print_waiting()
    }
}

fn run_single_script(script_container: ScriptContainer, output_manager: Arc<OutputManager>) {
    let script_path = script_container.path.as_str();

    if !Path::new(script_path).exists() {
        output_manager.send(OutputCommand::Error {
            title: script_container.title.clone(),
            message: format!("{} does not exist", script_path),
        });
        return;
    }

    // Add task to running tasks
    output_manager.send(OutputCommand::AddTask(script_container.title.clone()));

    let start = Instant::now();
    let result = Command::new(script_path)
        .output()
        .expect("Failed to execute script");
    let duration = start.elapsed();

    let mut output = None;
    if !result.status.success() {
        let mut output_lines = Vec::new();
        if !result.stdout.is_empty() {
            output_lines.push("Output:".to_string());
            output_lines.extend(
                String::from_utf8_lossy(&result.stdout)
                    .lines()
                    .map(|s| s.to_string()),
            );
        }
        if !result.stderr.is_empty() {
            output_lines.push("Error:".to_string());
            output_lines.extend(
                String::from_utf8_lossy(&result.stderr)
                    .lines()
                    .map(|s| s.to_string()),
            );
        }
        output = Some(output_lines.join("\n"));
    }

    output_manager.send(OutputCommand::CompleteTask {
        title: script_container.title.clone(),
        success: result.status.success(),
        duration,
        output,
    });
}

/// containing all the information needed to print user-friendly output.
struct ScriptContainer {
    /// title of the execution
    title: String,

    /// use emojis in printout or not
    use_emoji: bool,

    /// show execution time or now
    show_time: bool,

    /// path to the script
    path: String,
}

impl ScriptContainer {
    fn new(use_emoji: bool, show_time: bool, path: String) -> Self {
        let path_obj = Path::new(&path);
        let title = path_obj
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or(&path)
            .to_string();

        Self {
            title,
            use_emoji,
            show_time,
            path,
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
