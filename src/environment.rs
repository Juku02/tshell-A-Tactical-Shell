use regex::Regex;
use std::collections::HashMap;
use std::env;

pub struct Environment {
    env: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn get(&self, basic_sys: &str, input: &str) -> (String, Vec<String>) {
        let re_unix = Regex::new(r"\$(\w+)").unwrap(); // Unix-style: $VARIABLE
        let re_windows = Regex::new(r"%(\w+)%").unwrap();
        let input_with_env = if basic_sys == "windows" {
            re_windows.replace_all(&input, |caps: &regex::Captures| {
                let var_name = &caps[1];
                self.env
                    .get(var_name)
                    .cloned()
                    .unwrap_or_else(|| "".to_string())
            })
        } else {
            re_unix.replace_all(&input, |caps: &regex::Captures| {
                let var_name = &caps[1];
                self.env
                    .get(var_name)
                    .cloned()
                    .unwrap_or_else(|| "".to_string())
            })
        };
        let parts: Vec<&str> = input_with_env.split_whitespace().collect();
        let command_name = parts[0].to_string(); // Convert command name to String
        let args: Vec<String> = if basic_sys == "windows" {
            parts[1..]
                .iter()
                .map(|arg| {
                    // Replace environment variables in arguments only
                    re_windows.replace_all(arg, |caps: &regex::Captures| {
                        let var_name = &caps[1];
                        self.env
                            .get(var_name)
                            .cloned()
                            .unwrap_or_else(|| "".to_string())
                    })
                    .to_string()
                })
                .collect()
        } else {
            parts[1..]
                .iter()
                .map(|arg| {
                    // Replace environment variables in arguments only
                    re_unix.replace_all(arg, |caps: &regex::Captures| {
                        let var_name = &caps[1];
                        self.env
                            .get(var_name)
                            .cloned()
                            .unwrap_or_else(|| "".to_string())
                    })
                    .to_string()
                })
                .collect()
        };
        (command_name.to_string(), args)
    }

    pub fn load(&mut self) {
        self.env.extend(env::vars());
    }
}
