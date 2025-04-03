
use std::fs::OpenOptions;
use std::fmt;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warn => write!(f, "Warn"),
            LogLevel::Error => write!(f, "Error"),
        }
    }
}

#[derive(Clone)]
pub struct Logger {
    log_file: Box<Path>,
}
impl Logger {
    pub fn new(log_file: Box<Path>) -> Logger {
        Logger { 
            log_file: log_file,
        }
    }

    pub fn log(&self, level: LogLevel, callback: &str, command: &str) {
        let log_message = format!("[{:?}] {}: {}", level, command, callback);
        if let Err(e) = self.write_to_file(&log_message) {
            eprintln!("Failed to write to log file: {}", e);
        }
        
    }
    fn write_to_file(&self, message: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;
        file.write_all(message.as_bytes())
    }
}
