use crate::command::Command;
use crate::environment::Environment;
use crate::log::Logger;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
pub struct Shell {
    logger: Logger,
    env: Environment,
    basic_sys: String,
}

fn signal_handler(running: Arc<AtomicBool>, input: &mut String) {
    // Handle Ctrl+C signal
    let running_clone = Arc::clone(&running);
    let mut input_clone = input.clone();
    ctrlc::set_handler(move || {
        print!("\ntshell> ");
        io::stdout().flush().expect("Failed to flush stdout");
        input_clone.clear(); // Clear the input buffer to reset the shell state
        running_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");
}

impl Shell {
    pub fn new(logger: Logger, env: Environment) -> Self {
        Shell {
            logger,
            env,
            basic_sys: std::env::consts::OS.to_string(),
        }
    }

    pub fn run(&mut self) {
        let running = Arc::new(AtomicBool::new(true));
        let mut input = String::new();

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        signal_handler(Arc::clone(&running), &mut input);

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
            
            
            let (command_name, args) = self.env.get(&self.basic_sys, &input);
            let command = Command::new(command_name.to_string(), self.logger.clone());
            command.execute(args);
        }
    }
}
