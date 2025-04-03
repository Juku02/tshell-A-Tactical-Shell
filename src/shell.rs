
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
        let ctrlc_triggered = Arc::new(AtomicBool::new(false));
        let ctrlc_clone = Arc::clone(&ctrlc_triggered);

        // Set up Ctrl+C handler
        ctrlc::set_handler(move || {
            ctrlc_clone.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl+C handler");

        while running.load(Ordering::SeqCst) {
            print!("tshell> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read input");
                continue;
            }

            let input = input.trim();
            if input.is_empty() {
                if ctrlc_triggered.load(Ordering::SeqCst) {
                    ctrlc_triggered.store(false, Ordering::SeqCst);
                    continue;
                }
                continue;
            }
            else{
                if input == "exit" {
                    println!("Exiting tshell...");
                    break;
                }

                let parts: Vec<&str> = input.split_whitespace().collect();
                let command_name = parts[0];
                let args = parts[1..].to_vec();

                let command = Command::new(command_name.to_string(), self.logger.clone());
                command.execute(args);
            }
        }
    }
}