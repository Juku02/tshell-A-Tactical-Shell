mod log;
mod command;
mod shell;

use log::Logger;
use shell::Shell;

fn main() {
    let logger = Logger::new(Box::from(std::path::Path::new("test.log").to_path_buf()));
    let mut shell = Shell::new(logger);
    shell.run();
}
