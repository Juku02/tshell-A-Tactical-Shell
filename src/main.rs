mod log;
mod command;
mod shell;
mod environment;

use log::Logger;
use shell::Shell;
use environment::Environment;

fn main() {
    let logger = Logger::new(Box::from(std::path::Path::new("test.log").to_path_buf()));
    let mut env = Environment::new();
    env.load(); // Load environment variables once

    let mut shell = Shell::new(logger, env);
    shell.run();
}
