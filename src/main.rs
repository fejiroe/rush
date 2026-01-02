use std::{
    env, fs,
    io::{self, Error, Write},
    os::unix::fs::PermissionsExt,
    process::{self, Command, ExitStatus},
};

struct Shell {
    prompt: String,
    input: String,
    cmd: Option<String>,
    arg: Option<String>,
    cwd: Option<String>,
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
        let path = env::var("PATH").unwrap_or_else(|_| String::new());
        for dir in path.split(':') {
            let path = std::path::Path::new(dir).join(cmd);
            if path.exists() && Self::is_executable(&path) {
                return Some(path);
            }
        }
        None
    }
    fn print_prompt(&self) {
        print!("{}", &self.prompt);
        io::stdout().flush().expect("fail to flush stdout");
    }
    fn check_builtin(&self) -> bool {
        let builtins: std::collections::HashSet<&str> =
            ["cd", "echo", "exit", "pwd", "type"].into();
        self.cmd.as_deref().map_or(false, |c| builtins.contains(c))
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
    fn exec_pwd(&mut self) -> bool {
        self.cwd = Self::get_dir();
        print!("{}", &self.cwd.clone().unwrap_or_else(|| String::new()));
        return true;
    }
    fn exec_type(&self) -> bool {
        // not properly working
        let cmd = match &self.cmd {
            Some(c) => c.as_str(),
            None => return false,
        };
        if Shell::check_builtin(&self) == true {
            println!("{} is a builtin", cmd);
            return true;
        }
        match Self::find_in_path(cmd) {
            Some(full) => {
                println!("{} is {}", cmd, full.display());
            }
            None => {
                println!("{} is not found", cmd);
            }
        }
        true
    }
    fn exec_extern(&self) -> io::Result<ExitStatus> {
        let cmd: &String = self.cmd.as_ref().unwrap();
        let mut child = Command::new(cmd);
        if let Some(ref args) = self.arg {
            for a in args.split_whitespace() {
                child.arg(a);
            }
        }
        child.status()
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
    fn handle_in(&mut self) {
        if self.cmd.is_none() {
            return;
        }
        if self.check_builtin() {
            self.exec_builtin();
        }
        match self.exec_extern() {
            Ok(status) => {
                if !status.success() {
                    eprintln!(
                        "{}: exit status {}",
                        self.cmd.as_deref().unwrap_or(""),
                        status
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "{}: failed to execute: {}",
                    self.cmd.as_deref().unwrap_or(""),
                    e
                );
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let mut shell = Shell {
        prompt: String::from("$  "),
        input: String::new(),
        cmd: None,
        arg: None,
        cwd: None,
    };
    loop {
        shell.print_prompt();
        if shell.read_ln()? {
            break;
        }
        shell.parse_in();
        shell.handle_in();
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
        };
        shell.cmd = Some("echo".to_string());
        assert!(shell.check_builtin());
        shell.cmd = Some("ls".to_string());
        assert!(!shell.check_builtin());
    }
}
