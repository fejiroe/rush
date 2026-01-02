use std::{
    env, fs,
    io::{self, Error, Write},
    os::unix::fs::PermissionsExt,
    process::{self, Command, Stdio},
};

struct Shell {
    prompt: String,
    input: String,
    cmd: Option<String>,
    arg: Option<String>,
    cwd: Option<String>,
    exit_status: Option<i32>,
}

impl Shell {
    fn get_dir() -> Option<String> {
        env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(String::from))
    }
    fn is_executable(path: &std::path::Path) -> bool {
        match fs::metadata(path) {
            Ok(metadata) => metadata.is_file() && metadata.permissions().mode() & 0o111 != 0,
            Err(_) => false,
        }
    }
    fn find_in_path(cmd: &str) -> Option<std::path::PathBuf> {
        if let Some(path) = env::var_os("PATH") {
            for dir in env::split_paths(&path) {
                let candidate = std::path::Path::new(&dir).join(cmd);
                if candidate.exists() && Self::is_executable(&candidate) {
                    return Some(candidate);
                }
            }
        }
        None
    }
    fn has_starship() -> bool {
        if !Self::find_in_path("starship").is_none() {
            return true;
        } else {
            return false;
        };
    }
    fn update_prompt(&mut self) -> io::Result<()> {
        if !Self::has_starship() {
            self.prompt = String::from("$ ");
        } else {
            let child = Command::new("starship")
                .arg("prompt")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()?;
            let output = child.wait_with_output()?;
            self.prompt = String::from_utf8_lossy(&output.stdout)
                .to_string()
                .chars()
                .filter(|c| !['{', '}', '%'].contains(c))
                .collect();
        }
        Ok(())
    }
    fn print_prompt(&self) {
        print!("{}", &self.prompt);
        io::stdout().flush().expect("fail to flush stdout");
    }
    fn check_builtin(word: &str) -> bool {
        let builtins: std::collections::HashSet<&str> =
            ["cd", "echo", "exit", "pwd", "type"].into();
        builtins.contains(word)
    }
    fn read_ln(&mut self) -> io::Result<bool> {
        self.input.clear();
        let confirm = io::stdin().read_line(&mut self.input)?;
        Ok(confirm > 0)
    }
    fn exec_cd(&mut self) -> bool {
        if let Some(ref path) = self.arg {
            match env::set_current_dir(path) {
                Ok(_) => {
                    self.cwd = Self::get_dir();
                }
                Err(e) => {
                    eprint!("cd: {}: {}", path, e);
                }
            }
            true
        } else {
            true
        }
    }
    fn exec_echo(&self) -> bool {
        if let Some(ref a) = self.arg {
            println!("{}", a);
        } else {
            println!();
        }
        true
    }
    fn exec_kill(&mut self) -> bool {
        let pid = match &self.arg {
            Some(arg) => arg.split_whitespace().next(),
            None => None,
        };
        let pid = match pid {
            Some(p) => p,
            None => {
                eprintln!("kill: missing arg");
                return false;
            }
        };
        match Command::new("kill").arg(pid).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("kill: failed to kill process");
                }
                self.exit_status = Some(status.code().unwrap_or(0))
            }
            Err(e) => {
                eprintln!("kill: failed to kill process: {}", e);
            }
        }
        true
    }
    fn exec_pwd(&mut self) -> bool {
        self.cwd = Self::get_dir();
        print!("{}", &self.cwd.clone().unwrap_or_else(|| String::new()));
        return true;
    }
    fn exec_type(&self) -> bool {
        let target = match &self.arg {
            Some(arg) => arg.split_whitespace().next().unwrap_or_default(),
            None => return false,
        };
        if target.is_empty() {
            println!("type: missing arg");
            return true;
        }
        if Shell::check_builtin(target) == true {
            println!("{} is a builtin", target);
            return true;
        }
        match Self::find_in_path(target) {
            Some(full) => {
                println!("{} is {}", target, full.display());
            }
            None => {
                println!("{} is not found", target);
            }
        }
        true
    }
    fn exec_extern(&mut self) -> io::Result<()> {
        let cmd: &String = self.cmd.as_ref().unwrap();
        let mut child = Command::new(cmd);
        if let Some(ref args) = self.arg {
            for a in args.split_whitespace() {
                child.arg(a);
            }
        }
        let status = child.status()?;
        if !status.success() {
            eprintln!("{}: exit status {}", cmd, status)
        }
        self.exit_status = Some(status.code().unwrap_or(0));
        Ok(())
    }
    fn exec_builtin(&mut self) -> bool {
        let cmd = match &self.cmd {
            Some(c) => c.as_str(),
            None => return false,
        };
        match cmd {
            "cd" => self.exec_cd(),
            "echo" => self.exec_echo(),
            "exit" => {
                process::exit(0);
            }
            "kill" => self.exec_kill(),
            "pwd" => self.exec_pwd(),
            "type" => self.exec_type(),
            _ => false,
        }
    }
    fn parse_in(&mut self) {
        if self.input.is_empty() {
            self.cmd = None;
            self.arg = None;
            return;
        }
        let trimmed = self.input.trim();
        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let cmd = parts.next().unwrap_or("").to_string();
        self.cmd = Some(cmd);
        self.arg = parts.next().map(|s| s.to_string());
    }
    fn handle_in(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.cmd.is_none() {
            return Ok(());
        }
        if Shell::check_builtin(&self.cmd.as_deref().unwrap_or("")) {
            self.exec_builtin();
            return Ok(());
        } else {
            self.exec_extern()?;
            Ok(())
        }
    }
}

fn main() -> Result<(), Error> {
    let mut shell = Shell {
        prompt: String::new(),
        input: String::new(),
        cmd: None,
        arg: None,
        cwd: None,
        exit_status: None,
    };
    shell.update_prompt()?;
    loop {
        shell.print_prompt();
        if shell.read_ln()? {
            shell.parse_in();
            shell.handle_in();
            shell.update_prompt()?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_basic() {
        let mut shell = Shell {
            prompt: String::from("$"),
            input: "echo hello".to_string(),
            cmd: None,
            arg: None,
            cwd: Some(String::new()),
            exit_status: None,
        };
        shell.parse_in();
        assert_eq!(shell.cmd.as_deref(), Some("echo"));
        assert_eq!(shell.arg.as_deref(), Some("hello"));
    }
    #[test]
    fn test_check_builtin() {
        let mut shell = Shell {
            prompt: String::default(),
            input: String::new(),
            cmd: None,
            arg: None,
            cwd: Some(String::new()),
            exit_status: None,
        };
        shell.cmd = Some("echo".to_string());
        assert!(Shell::check_builtin(&shell.cmd.as_deref().unwrap()));
        shell.cmd = Some("ls".to_string());
        assert!(!Shell::check_builtin(&shell.cmd.as_deref().unwrap()));
    }
    #[test]
    fn test_find_in_path() {
        let path = Shell::find_in_path("ls");
        assert!(path.is_some(), "ls not found in PATH");
    }
}
