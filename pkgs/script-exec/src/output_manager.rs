use crate::printer::EmojiPrinter;
use crate::printer::Printer;
use crate::printer::SystemdPrinter;
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
    pub fn new(printer_type: PrinterTypes) -> Self {
        let (sender, receiver) = channel();

        // Create the printer based on type
        let printer: Box<dyn Printer + Send> = match printer_type {
            PrinterTypes::Emoji => Box::new(EmojiPrinter),
            PrinterTypes::Systemd => Box::new(SystemdPrinter),
        };

        // Spawn the output thread
        thread::spawn(move || {
            let mut display_state = DisplayState {
                running_tasks: Vec::new(),
                printer,
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
                            display_state.printer.success(&title, duration)
                        } else {
                            display_state.printer.failure(&title, output, duration)
                        };
                        display_state.remove_task(&title, result_output);
                    }
                    OutputCommand::Error { title, message } => {
                        display_state.add_error_output(display_state.printer.failure(
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

    pub fn send(&self, command: OutputCommand) {
        self.sender.send(command).unwrap();
    }
}

// Update DisplayState to use dynamic dispatch
struct DisplayState {
    running_tasks: Vec<RunningTask>,
    printer: Box<dyn Printer + Send>,
}

// Update DisplayState implementation
impl DisplayState {
    fn add_task(&mut self, title: String) {
        self.clear_waiting();
        self.running_tasks.push(RunningTask { title });
        self.print_waiting();
    }

    fn remove_task(&mut self, title: &str, result_output: String) {
        self.clear_waiting();
        self.running_tasks.retain(|task| task.title != title);
        println!("{}", result_output);
        self.print_waiting();
    }

    fn print_waiting(&self) {
        if !self.printer.print_waiting() {
            return;
        }
        for task in &self.running_tasks {
            println!("{}", self.printer.waiting(&task.title))
        }
    }

    fn add_error_output(&mut self, output: String) {
        self.clear_waiting();
        println!("{}", output);
        self.print_waiting();
    }

    // clear_waiting remains the same
    fn clear_waiting(&self) {
        if !self.printer.print_waiting() {
            return;
        }
        let mut stdout = stdout();
        let total_lines = self.running_tasks.len();
        for _ in 0..total_lines {
            stdout.execute(cursor::MoveUp(1)).unwrap();
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        }
    }
}

struct RunningTask {
    title: String,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrinterTypes {
    Emoji,
    Systemd,
    // Prometheus,
}
