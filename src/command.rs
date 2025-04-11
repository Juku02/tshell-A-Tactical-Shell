use crate::log::{LogLevel, Logger};
use std::process::Command as ProcessCommand;
pub struct Command{
    pub name: String,
    pub logger: Logger,
}

impl Command {
    pub fn new(name: String, logger: Logger) -> Command {
        Command {
            name: name,
            logger: logger,
        }
    }
    pub fn execute(&self, args: Vec<String>) {
        let mut command = if cfg!(target_os = "windows") {
            let mut cmd = ProcessCommand::new("cmd");
            cmd.arg("/C");
            cmd
        } else {
            let mut cmd = ProcessCommand::new("sh");
            cmd.arg("-c");
            cmd
        };
        match command.arg(&self.name).args(args).output() {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    print!("{}", stdout);
                    self.logger.log(LogLevel::Info, &stdout, &self.name);
                }
                if !output.stderr.is_empty() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    self.logger.log(LogLevel::Warn, &stderr, &self.name);
                }
            }
            Err(e) => {
                let error_message = format!("Failed to execute command '{}': {}", self.name, e);
                self.logger.log(LogLevel::Error, &error_message, &self.name);
            }
        }
    }
}