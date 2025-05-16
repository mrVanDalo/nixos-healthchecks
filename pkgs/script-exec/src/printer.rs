use crossterm::style::Stylize;
use std::time::Duration;

pub trait Printer: Send {
    fn print_waiting(&self) -> bool;
    fn success(&self, title: &String, duration: Duration) -> String;
    fn failure(&self, title: &String, output: Option<String>, duration: Duration) -> String;
    fn waiting(&self, title: &String) -> String;
}

pub struct EmojiPrinter;

impl Printer for EmojiPrinter {
    fn print_waiting(&self) -> bool {
        true
    }

    fn success(&self, title: &String, duration: Duration) -> String {
        format!("✅ {} [{:.2?}s]", title, duration.as_secs_f64())
    }

    fn failure(&self, title: &String, output: Option<String>, duration: Duration) -> String {
        let mut result = format!("❌ {} [{:.2?}s]", title, duration.as_secs_f64());
        if let Some(output) = output {
            result.push('\n');
            result.push_str(&output);
        }
        result
    }

    fn waiting(&self, title: &String) -> String {
        format!("⏳ {}", title)
    }
}

pub struct SystemdPrinter;

impl Printer for SystemdPrinter {
    fn print_waiting(&self) -> bool {
        true
    }

    fn success(&self, title: &String, duration: Duration) -> String {
        format!(
            "{} {} [{:.2?}s]",
            "[ OK ]".green(),
            title,
            duration.as_secs_f64()
        )
    }

    fn failure(&self, title: &String, output: Option<String>, duration: Duration) -> String {
        let mut result = format!(
            "{} {} [{:.2?}s]",
            "[FAIL]".red(),
            title,
            duration.as_secs_f64()
        );
        if let Some(output) = output {
            result.push('\n');
            result.push_str(&output);
        }
        result
    }

    fn waiting(&self, title: &String) -> String {
        format!("{} {}", "[WAIT]".yellow(), title)
    }
}

pub struct PrometheusPrinter;

impl Printer for PrometheusPrinter {
    fn print_waiting(&self) -> bool {
        false
    }

    fn success(&self, title: &String, duration: Duration) -> String {
        format!(
            r#"nixos_healthcheck_status{{check="{}", status="success"}} 1
nixos_healthcheck_duration_seconds{{check="{}", status="success"}} {}"#,
            title,
            title,
            duration.as_secs_f64()
        )
    }

    fn failure(&self, title: &String, _output: Option<String>, duration: Duration) -> String {
        let result = format!(
            r#"nixos_healthcheck_status{{check="{}", status="failure"}} 0
nixos_healthcheck_duration_seconds{{check="{}", status="failure"}} {}"#,
            title,
            title,
            duration.as_secs_f64()
        );
        result
    }

    fn waiting(&self, _title: &String) -> String {
        String::new()
    }
}
