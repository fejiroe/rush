use std::{
    env, fs,
    io::{self, Error, Write},
    path::PathBuf,
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
    path: Option<String>,
    cwd: String,
}

impl Shell {
    fn print_prompt(&self) {
        println!("{}", &self.prompt);
    }
    fn check_builtin(&self) -> bool {
        let builtins: std::collections::HashSet<&str> =
            ["cd", "echo", "exit", "pwd", "type"].into();
        return true; // do real check !!!!!!!
    }
    fn read_ln(&mut self) {
        io::stdin().read_line(&mut self.input).expect("input error");
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
            // "cd" => {}
            "echo" => {
                if let Some(ref a) = self.arg {
                    println!("{}", a);
                    return true;
                } else {
                    return false;
                }
            }
            "exit" => {
                process::exit(0);
            }
            "pwd" => {
                self.cwd = get_dir();
                print!("{}", &self.cwd);
                return true;
            }
            // "type" => {}
            &_ => todo!(),
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
        prompt: String::from("$"),
        input: String::new(),
        cmd: None,
        arg: None,
        path: None,
        cwd: String::new(),
    };
    loop {
        shell.print_prompt();
        shell.read_ln();

        shell.parse_in();
        //shell.handle_in();
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
            path: None,
            cwd: String::new(),
        };
        shell.parse_in();
        assert_eq!(shell.cmd.as_deref(), Some("echo"));
        assert_eq!(shell.arg.as_deref(), Some("hello"));
    }
}
