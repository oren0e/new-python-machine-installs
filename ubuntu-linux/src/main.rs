mod utils;
mod environment;

use crate::utils::{git_clone, apt_install, apt_update,
                   write_to_file, pip_install, pipx_install};
use crate::environment::Environment;
use std::process::{Command, Stdio, exit};
use std::error::Error;
use std::env;


fn main() -> Result<(), Box<dyn Error>> {
    apt_update().unwrap_or_else(|error| {
        println!("Update failed with error: {}", error);
        exit(1)
    });
    apt_install(vec!["lsb-release"], Some(vec!["-y"])).unwrap_or_else(|error| {
        println!("Program failed with error: {}", error);
        exit(1)
    });

    let env_vars: Environment = Environment::load().unwrap_or_else(|error| {
        println!("Failed getting environment variable with error: {}", error);
        exit(1)
    });

    apt_install(vec!["zsh", "curl", "wget", "git"], Some(vec!["-y"])).unwrap_or_else(|error| {
        println!("Program failed with error: {}", error);
        exit(1)});

    let mut c = Command::new("wget")
        .arg("https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh")
        .spawn()?;
    c.wait().unwrap_or_else(|error| {
        println!("wget failed with error: {}", error);
        exit(1)
    });

    git_clone("https://github.com/junegunn/fzf.git", format!("{}/.fzf", env_vars.home_var).as_str()).unwrap_or_else(|error| {
        println!("Git clone failed with error: {}", error);
        exit(1)
    });

    let mut c = Command::new(format!("{}/.fzf/install", env_vars.home_var).as_str())
        .arg("--all")
        .spawn()?;
    c.wait().unwrap_or_else(|error| {
        println!("Failed to install fzf with error: {}", error);
        exit(1)
    });

    apt_update().unwrap_or_else(|error| {
        println!("Update failed with error: {}", error);
        exit(1)
    });

    env::set_var("DEBIAN_FRONTEND", "noninteractive");

    apt_install(vec!["tzdata"], Some(vec!["-y", "--no-install-recommends"])).unwrap_or_else(|error| {
        println!("Program failed with error: {}", error);
        exit(1)
    });

    apt_install(vec!["make", "build-essential", "libssl-dev", "zlib1g-dev", "libbz2-dev",
                     "libreadline-dev", "libsqlite3-dev", "llvm",
                     "libncurses5-dev", "xz-utils", "tk-dev",
                     "libxml2-dev", "libxmlsec1-dev", "libffi-dev", "liblzma-dev"], Some(vec!["-y", "--no-install-recommends"])).unwrap_or_else(|error| {
        println!("Program failed with error: {}", error);
        exit(1)
    });

    git_clone("https://github.com/pyenv/pyenv.git", format!("{}/.pyenv", env_vars.home_var).as_str()).unwrap_or_else(|error| {
        println!("Git clone failed with error: {}", error);
        exit(1)
    });

    write_to_file(format!("{}/.bashrc", env_vars.home_var).as_str(), format!("export PYENV_ROOT=\"{}/.pyenv\"", env_vars.home_var).as_str()).unwrap_or_else(|error| {
        println!("Writing to file failed with error: {}", error);
        exit(1)
    });
    write_to_file(format!("{}/.bashrc", env_vars.home_var).as_str(), format!("export PATH=\"{}/bin:{}\"",format!("{}/.pyenv", env_vars.home_var), env_vars.path_var).as_str()).unwrap_or_else(|error| {
        println!("Writing to file failed with error: {}", error);
        exit(1)
    });
    write_to_file(format!("{}/.bashrc", env_vars.home_var).as_str(), "if command -v pyenv 1>/dev/null 2>&1; then\n  eval \"$(pyenv init -)\"\nfi").unwrap_or_else(|error| {
        println!("Writing to file failed with error: {}", error);
        exit(1)
    });

    let mut c = Command::new(format!("{}/.pyenv/bin/pyenv", env_vars.home_var).as_str())
        .args(&["install", env_vars.python_version])
        .spawn()?;
    c.wait().unwrap_or_else(|error| {
        println!("pyenv install failed with error: {}", error);
        exit(1)
    });

    pip_install(vec!["pip"], Some(vec!["-U"])).unwrap_or_else(|error| {
        println!("pip install failed with error: {}", error);
        exit(1)
    });

    pip_install(vec!["pipx"], Some(vec!["--user"])).unwrap_or_else(|error| {
        println!("pip install failed with error: {}", error);
        exit(1)
    });

    let mut c = Command::new(format!("{}/.pyenv/versions/{}/bin/python3", env_vars.home_var, env_vars.python_version).as_str())
        .args(&["-m", "pipx", "ensurepath"])
        .spawn()?;
    c.wait().unwrap_or_else(|error| {
        println!("pipx ensurepath failed with error: {}", error);
        exit(1)
    });

    pipx_install(vec!["poetry"]).unwrap_or_else(|error| {
        println!("pipx install failed with error: {}", error);
        exit(1)
    });

    pipx_install(vec!["pipenv"]).unwrap_or_else(|error| {
        println!("pipx install failed with error: {}", error);
        exit(1)
    });

    apt_update().unwrap_or_else(|error| {
        println!("Update failed with error: {}", error);
        exit(1)
    });

    apt_install(vec!["apt-transport-https", "ca-certificates", "curl", "gnupg"], Some(vec!["-y"])).unwrap_or_else(|error| {
        println!("Program failed with error: {}", error);
        exit(1)
    });

    let mut c = Command::new("curl")
        .args(&["-fsSL", "https://download.docker.com/linux/ubuntu/gpg"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(prior_command) = c.stdout.take() { // if it's not None
        let mut c2 = Command::new("gpg")
            .args(&["--dearmor", "-o", "/usr/share/keyrings/docker-archive-keyring.gpg"])
            .stdin(prior_command)
            .spawn()?;
        c2.wait().unwrap_or_else(|error| {
            println!("GPG keyring error: {}", error);
            exit(1)
        });
    }

    apt_update().unwrap_or_else(|error| {
        println!("Update failed with error: {}", error);
        exit(1)
    });

    write_to_file("/etc/apt/sources.list.d/docker.list",
                  format!("deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu {:?} stable", env_vars.ubuntu_distribution).as_str()).unwrap_or_else(|error| {
        println!("Writing to file failed with error: {}", error);
        exit(1)
    });

    apt_update().unwrap_or_else(|error| {
        println!("Update failed with error: {}", error);
        exit(1)
    });

    apt_install(vec!["docker-ce", "docker-ce-cli", "containerd.io"], Some(vec!["-y"])).unwrap_or_else(|error| {
        println!("Program failed with error: {}", error);
        exit(1)
    });

    apt_install(vec!["vim"], Some(vec!["-y"])).unwrap_or_else(|error| {
        println!("Program failed with error: {}", error);
        exit(1)
    });
    Ok(())
}
