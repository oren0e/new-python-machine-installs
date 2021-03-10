use std::process::{Command, Stdio, exit};
use std::env;

pub struct Environment<'a> {
    pub home_var: String,
    pub path_var: String,
    pub python_version: &'a str,
    pub ubuntu_distribution: String
}

impl Environment<'_> {
    pub fn new() -> Environment<'static> {
        let home_var = env::var("HOME").unwrap_or_else(|error| {
                println!("Failed getting HOME environment variable with error: {}", error);
                exit(1)
            });
        let path_var = env::var("PATH").unwrap_or_else(|error| {
                println!("Failed getting PATH environment variable with error: {}", error);
                exit(1)
            });
       let python_version = "3.9.0";
        let c = Command::new("lsb_release")
            .arg("-cs")
            .stdout(Stdio::piped())
            .output()
            .unwrap_or_else(|error| {
                println!("Failed running lsb_release with error: {}", error);
                exit(1)
            });
        let mut ubuntu_distribution = String::from_utf8(c.stdout).unwrap_or_else(|error| {
            println!("Failed reading from output with error: {}", error);
            exit(1)
        });
        ubuntu_distribution.pop();
        Environment {
            home_var,
            path_var,
            python_version,
            ubuntu_distribution
        }
    }
}