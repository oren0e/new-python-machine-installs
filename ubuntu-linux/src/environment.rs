use std::process::{Command, Stdio};
use std::env;
use anyhow::Result;

pub struct Environment {
    pub home_var: String,
    pub path_var: String,
    pub python_version: String,
    pub ubuntu_distribution: String
}

impl Environment {
    pub fn load() -> Result<Environment> {
        let home_var = env::var("HOME")?;
        let path_var = env::var("PATH")?;
        let python_version = "3.9.0".to_owned();

        let c = Command::new("lsb_release")
            .arg("-cs")
            .stdout(Stdio::piped())
            .output()?;

        let mut ubuntu_distribution = String::from_utf8(c.stdout)?;
        ubuntu_distribution.pop();
        Ok(Environment {
            home_var,
            path_var,
            python_version,
            ubuntu_distribution
        })
    }
}