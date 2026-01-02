use std::{
    env, fs,
    io::{self, Error, Write},
    process::{self, Command, ExitStatus},
};

fn get_dir() -> String {
    // not actually an effective check?
    let binding = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("?"));
    let cwd = binding;
    return cwd.display().to_string();
}

struct Shell {
    prompt: String,
    input: String,
    cmd: Option<String>,
    arg: Option<String>,
    cwd: String,
}

impl Shell {
    fn print_prompt(&self) {
        print!("{}", &self.prompt);
        io::stdout().flush().expect("fail to flush stdout");
    }
    fn check_builtin(&self) -> bool {
        let builtins: std::collections::HashSet<&str> =
            ["cd", "echo", "exit", "pwd", "type"].into();
        self.cmd.as_deref().map_or(false, |c| builtins.contains(c))
    }
    fn read_ln(&mut self) {
        self.input.clear();
        io::stdin().read_line(&mut self.input).expect("input error");
    }
    fn exec_cd(&mut self) -> bool {
        if let Some(ref path) = self.arg {
            match env::set_current_dir(path) {
                Ok(_) => {
                    self.cwd = get_dir();
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
        self.cwd = get_dir();
        print!("{}", &self.cwd);
        return true;
    }
    fn exec_type(&self) -> bool {
        return true;
    }
    fn exec_extern(&self) -> io::Result<ExitStatus> {
        // unconfirmed
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
        // unconfirmed
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
        if Shell::check_builtin(self) == true {
            Shell::exec_builtin(self);
        } else {
            Shell::exec_extern(self);
        }
    }
}

fn main() -> Result<(), Error> {
    let mut shell = Shell {
        prompt: String::from("$  "),
        input: String::new(),
        cmd: None,
        arg: None,
        cwd: String::new(),
    };
    loop {
        shell.print_prompt();
        shell.read_ln();

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
            cwd: String::new(),
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
            cwd: String::new(),
        };
        shell.cmd = Some("echo".to_string());
        assert!(shell.check_builtin());
        shell.cmd = Some("ls".to_string());
        assert!(!shell.check_builtin());
    }
}
