use crate::environment::Environment;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::error::Error;

pub fn write_to_file(filename: &str, text: &str) -> Result<(), Box<dyn Error>>{
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)?;
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
        Ok(())
    }

pub fn apt_update() -> Result<(), Box<dyn Error>> {
    let mut c = Command::new("apt-get")
        .arg("update")
        .spawn()?;
    c.wait()?;
    Ok(())
}

pub fn apt_install<'a>(mut packages: Vec<&'a str>, options: Option<Vec<&'a str>>) -> Result<(), Box<dyn Error>>{
    let mut cmd_line = vec!["install"];
    cmd_line.append(&mut packages);
    cmd_line.append(&mut options.unwrap_or(vec![""]));
    let mut c = Command::new("apt-get")
        .args(cmd_line)
        .spawn()?;
    c.wait()?;
    Ok(())
}

pub fn git_clone(repo_address: &str, destination: &str) -> Result<(), Box<dyn Error>> {
    let mut c = Command::new("git")
        .args(&["clone", "--depth=1", repo_address, destination])
        .spawn()?;
    c.wait()?;
    Ok(())
}

pub fn pip_install<'a>(mut packages: Vec<&'a str>, options: Option<Vec<&'a str>>) -> Result<(), Box<dyn Error>> {
    let env_vars: Environment = Environment::new();
    let mut cmd_line = vec!["-m", "pip", "install"];
    cmd_line.append(&mut options.unwrap_or(vec![""]));
    cmd_line.append(&mut packages);
    let mut c = Command::new(format!("{}/.pyenv/versions/{}/bin/python3", env_vars.home_var, env_vars.python_version).as_str())
        .args(cmd_line)
        .spawn()?;
    c.wait()?;
    Ok(())
}

pub fn pipx_install (mut packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let env_vars: Environment = Environment::new();
    let mut c = Command::new(format!("{}/.local/bin/pipx", env_vars.home_var).as_str())
        .arg("install")
        .args(&mut packages)
        .spawn()?;
    c.wait()?;
    Ok(())
}
