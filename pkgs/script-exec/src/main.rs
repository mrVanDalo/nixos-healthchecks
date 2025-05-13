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
use std::sync::atomic::{AtomicBool, Ordering};

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

    /// The alternating titles and paths to the scripts (title=path)
    #[arg(value_parser = parse_title_path_pair)]
    pairs: Vec<(String, String)>,
}

fn parse_title_path_pair(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 {
        return Err("Each pair must be in the format 'title=path'".to_string());
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}



fn main() {
    env_logger::init();
    let args = Args::parse();

    if args.pairs.is_empty() {
        eprintln!("No paths provided");
        exit(1);
    }

    let output_manager = Arc::new(OutputManager::new(args.emoji, args.time));

    // Create ScriptContainers before spawning threads

    let mut handles = vec![];
    let mut scripts = args.pairs
        .into_iter()
        .map(|(title, path)| Script { title, path })
        .collect::<Vec<Script>>();
    scripts.reverse();

    let scripts = Arc::new(Mutex::new(scripts));

    // Near the start of main(), after creating output_manager:
    let all_successful = Arc::new(AtomicBool::new(true));

    // Modify the thread spawning section to include all_successful:
    for _ in 0..args.jobs {
        let scripts_mutex = Arc::clone(&scripts);
        let output_manager = Arc::clone(&output_manager);
        let all_successful = Arc::clone(&all_successful);

        let handle = thread::spawn(move || loop {
            let script = {
                let mut script_mutex_guard = scripts_mutex.lock().unwrap();
                if script_mutex_guard.is_empty() {
                    break;
                }
                script_mutex_guard.pop().unwrap()
            };

            run_script(script, output_manager.clone(), all_successful.clone());
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // After all threads complete, exit with appropriate status
    if !all_successful.load(Ordering::SeqCst) {
        exit(1);
    }
}

fn run_script(script: Script, output_manager: Arc<OutputManager>, all_successful: Arc<AtomicBool>) {
    let script_path = script.path.as_str();

    if !Path::new(script_path).exists() {
        output_manager.send(OutputCommand::Error {
            title: script.title.clone(),
            message: format!("{} does not exist", script_path),
        });
        all_successful.store(false, Ordering::SeqCst);
        return;
    }

    output_manager.send(OutputCommand::AddTask(script.title.clone()));

    let start = Instant::now();
    let result = Command::new(script_path)
        .output()
        .expect("Failed to execute script");
    let duration = start.elapsed();

    let mut output = None;
    if !result.status.success() {
        all_successful.store(false, Ordering::SeqCst);
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
        title: script.title.clone(),
        success: result.status.success(),
        duration,
        output,
    });
}

/// containing all the information needed to print user-friendly output.
struct Script {
    /// title of the execution
    title: String,

    /// path to the script
    path: String,
}

impl Script {
    #[allow(dead_code)]
    fn new(path: String) -> Self {
        let path_obj = Path::new(&path);
        let title = path_obj
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or(&path)
            .to_string();

        Self { title, path }
    }
}
