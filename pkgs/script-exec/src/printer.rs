use crossterm::style::Stylize;
use indexmap::IndexMap;
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

pub struct PrometheusPrinter {
    pub labels: IndexMap<String, String>,
}

impl PrometheusPrinter {
    pub fn new(labels: IndexMap<String, String>) -> PrometheusPrinter {
        PrometheusPrinter { labels }
    }

    fn format_labels(&self, title: &String, status: &str) -> String {
        // concat [( "check" , title ) , ("status", status) ] with self.labels
        let mut labels = self.labels.clone();
        labels.insert("status".to_string(), status.to_string());
        labels.insert("check".to_string(), title.clone());
        labels.reverse();

        // join all labels
        labels
            .iter()
            .map(|(key, value)| format!("\"{}\"=\"{}\"", key, value))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

/// Formats a HashMap of labels into a string in the format `"key1"="value1","key2"="value2",...`
impl Printer for PrometheusPrinter {
    fn print_waiting(&self) -> bool {
        false
    }

    fn success(&self, title: &String, duration: Duration) -> String {
        format!(
            r#"nixos_healthcheck_status {{ {} }}  1
nixos_healthcheck_duration_seconds{{ {} }} {}"#,
            self.format_labels(title, "success"),
            self.format_labels(title, "success"),
            duration.as_secs_f64()
        )
    }

    fn failure(&self, title: &String, _output: Option<String>, duration: Duration) -> String {
        format!(
            r#"nixos_healthcheck_status{{ {} }} 0
nixos_healthcheck_duration_seconds{{ {} }} {}"#,
            self.format_labels(title, "failure"),
            self.format_labels(title, "failure"),
            duration.as_secs_f64()
        )
    }

    fn waiting(&self, _title: &String) -> String {
        String::new()
    }
}
