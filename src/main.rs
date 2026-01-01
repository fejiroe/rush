use std::{
    fs,
    env,
    path::PathBuf,
    io::{self, Write, Error},
    process::{Command, ExitStatus}
};

fn getCurrentDir() -> String { // not actually an effective check?
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
    fn checkBuiltin(){}
    fn printPrompt(&self) {println!("{}", &self.prompt);}
    fn readLine(&mut self) {
        io::stdin()
            .read_line(&mut self.input)
            .expect("input error");}
    fn execExternal(&self) -> io::Result<ExitStatus> {
        // unconfirmed
        let cmd: &String = self.cmd.as_ref().unwrap();
        let mut child = Command::new(cmd);
        if let Some(ref args) = self.arg {
            for a in args.split_whitespace(){child.arg(a);}
            } child.status()
    }
    fn execBuiltIn(&mut self) -> bool {
        // unconfirmed
        let cmd = match &self.cmd {
        Some (c) => c.as_str(),
        None => return false,
        };
        match cmd {
            // "cd" => {}
            "echo" => {if let Some(ref a) = self.arg {println!("{}", a); return true;} else {return false;}}
            "exit" => {return true;}
            "pwd" => {self.cwd = getCurrentDir(); print!("{}", &self.cwd); return true;}
            // "type" => {}
            &_ => todo!()
        }
    }
    /*
    fn parseInput(&self) {
        let trimmed = self.input.trim_end_matches('\n').trim();
        if trimmed.is_empty() {
            self.cmd = None;
            self.arg = None;
            return;
        }
        if let Some((first.is_space(), rest)) {
            self.cmd = Some(first.to_string());
        } else {
            self.cmd = Some(trimmed.to_string());
            self.arg = None;
        }
    }
    fn handleInput(&self) {
        if checkBuiltin(self.cmd) == true {execBuiltIn(self.cmd);}
    } else {execExternal(self.cmd);}
    */
}

fn main() -> Result <(), Error> {
    let mut shell = Shell {
        prompt: String::from("$"),
        input: String::new(),
        cmd: None,
        arg: None,
        path: None,
        cwd: String::new()};
    loop {
        shell.printPrompt();
        shell.readLine();
        
        //shell.parseInput();
        //shell.handleInput();
    } Ok (())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test1() {
    }
}
