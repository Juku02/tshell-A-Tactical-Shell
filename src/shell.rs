
use crate::command::Command;
use crate::log::Logger;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
pub struct Shell {
    logger: Logger,
}


impl Shell {
    pub fn new(logger: Logger) -> Self {
        Shell { logger }
    }
    
    pub fn run(&mut self) {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);
        let mut input = String::new();
        let mut input_clone = input.clone();

        let stdin = io::stdin();
        let mut stdout = io::stdout();
        ctrlc::set_handler(move || {
            print!("\ntshell> ");
            io::stdout().flush().expect("Failed to flush stdout");
            input_clone.clear(); // Clear the input buffer to reset the shell state
            running_clone.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl+C handler");

        
        while running.load(Ordering::SeqCst) {
            print!("tshell> ");
            stdout.flush().expect("Failed to flush stdout");
            
            input.clear();
            let mut buffer = String::new();
                    if let Ok(bytes) = stdin.read_line(&mut buffer) {
                        if bytes > 0 {
                            input = buffer.trim().to_string();
                        }
                    }

            if input.is_empty() {
                continue;
            }
            
            if input == "exit" {
                eprintln!("Exiting shell...");
                break;
            }
            
            
            // Split the command into name and arguments
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            let command_name = parts[0];
            let args = parts[1..].to_vec();
            
            // Create and execute the command
            let command = Command::new(command_name.to_string(), self.logger.clone());
            command.execute(args);
        }
    }
}