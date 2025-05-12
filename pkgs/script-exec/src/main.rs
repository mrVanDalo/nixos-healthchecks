use clap::Parser;
use env_logger;
use std::path::Path;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod output_manager;
use output_manager::OutputCommand;
use output_manager::OutputManager;

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

// todo : exit with 1 if one of the scripts does not exit with 0
fn main() {
    env_logger::init();
    let mut args = Args::parse();
    args.paths.reverse();

    if args.paths.is_empty() {
        eprintln!("No paths provided");
        exit(1);
    }

    let output_manager = Arc::new(OutputManager::new(args.emoji, args.time));

    // Create ScriptContainers before spawning threads
    let script_containers: Vec<ScriptContainer> = args
        .paths
        .into_iter()
        .map(|path| ScriptContainer::new(path))
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

    /// path to the script
    path: String,
}

impl ScriptContainer {
    fn new(path: String) -> Self {
        let path_obj = Path::new(&path);
        let title = path_obj
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or(&path)
            .to_string();

        Self {
            title,
            path,
        }
    }

}
