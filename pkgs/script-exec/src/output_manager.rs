use crossterm::{
    ExecutableCommand, cursor,
    terminal::{Clear, ClearType},
};
use std::io::stdout;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub enum OutputCommand {
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

pub struct OutputManager {
    sender: Sender<OutputCommand>,
}

// Output Thread manager
// handles stdout output
impl OutputManager {
    pub fn new(use_emoji: bool, show_time: bool) -> Self {
        let (sender, receiver) = channel();
        // Spawn the output thread
        thread::spawn(move || {
            let pretty_print = PrettyPrint {
                use_emoji,
                show_time,
            };
            let mut display_state = DisplayState::new(pretty_print.clone());
            let pretty_print = PrettyPrint {
                use_emoji,
                show_time,
            };
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
                        let result_output = if success {
                            pretty_print.success(&title, duration)
                        } else {
                            pretty_print.failure(&title, output, duration)
                        };
                        display_state.remove_task(&title, result_output);
                    }
                    OutputCommand::Error { title, message } => {
                        display_state.add_error_output(pretty_print.failure(
                            &title,
                            Some(message),
                            Duration::new(0, 0),
                        ));
                    }
                }
            }
        });

        Self { sender }
    }

    pub(crate) fn send(&self, command: OutputCommand) {
        self.sender.send(command).unwrap();
    }
}

#[derive(Clone)]
pub struct PrettyPrint {
    /// use emojis in printout or not
    use_emoji: bool,

    /// show execution time or now
    show_time: bool,
}

impl PrettyPrint {
    fn exit_code_visualization(&self) -> (&'static str, &'static str, &'static str) {
        if self.use_emoji {
            ("⏳", "✅", "❌")
        } else {
            ("[Wait]", "[ OK ]", "[Fail]")
        }
    }

    /// Return waiting line
    fn waiting(&self, title: &String) -> String {
        let (wait, _, _) = self.exit_code_visualization();
        format!("{} {}", wait, title)
    }

    /// Return success line
    fn success(&self, title: &String, duration: Duration) -> String {
        let (_, ok, _) = self.exit_code_visualization();
        if self.show_time {
            format!("{} {} [{:.2?}s]", ok, title, duration.as_secs_f64())
        } else {
            format!("{} {}", ok, title)
        }
    }

    /// Return failure line
    fn failure(&self, title: &String, output: Option<String>, duration: Duration) -> String {
        let (_, _, fail) = self.exit_code_visualization();
        let mut result = if self.show_time {
            format!("{} {} [{:.2?}s]", fail, title, duration.as_secs_f64())
        } else {
            format!("{} {}", fail, title)
        };

        if let Some(output) = output {
            result.push('\n');
            result.push_str(&output);
        }
        result
    }
}

struct DisplayState {
    pretty_print: PrettyPrint,

    // running tasks memory to print waiting  lines
    running_tasks: Vec<RunningTask>,
}

struct RunningTask {
    title: String,
}

impl DisplayState {
    fn new(pretty_print: PrettyPrint) -> Self {
        Self {
            pretty_print,
            running_tasks: Vec::new(),
        }
    }

    fn add_task(&mut self, title: String) {
        self.clear_waiting();
        self.running_tasks.push(RunningTask { title });
        self.print_waiting()
    }

    fn remove_task(&mut self, title: &str, result_output: String) {
        self.clear_waiting();
        self.running_tasks.retain(|task| task.title != title);
        println!("{}", result_output);
        self.print_waiting()
    }

    fn clear_waiting(&self) {
        let mut stdout = stdout();
        let total_lines = self.running_tasks.len();
        for _ in 0..total_lines {
            stdout.execute(cursor::MoveUp(1)).unwrap();
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        }
    }

    fn print_waiting(&self) {
        for task in &self.running_tasks {
            //println!("⏳ {}", task.title);
            println!("{}", self.pretty_print.waiting(&task.title))
        }
    }

    fn add_error_output(&mut self, output: String) {
        self.clear_waiting();
        println!("{}", output);
        self.print_waiting()
    }
}
