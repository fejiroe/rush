use std::io;
use std::io::Error;
use std::process::Command;

/*
fn getPathVar() {return std::env::var("HOME");}

fn getCurrentDir() {return env::current_dir();}

fn parseCmd(input: &str) -> Option<String> {
    if input.is_empty() {
        None
    } else {
    // else check for first space, return substring until space
    return String::from(input)?;
    }
}

fn parseArg(input: &str) -> Option<String> {
    if input is empty return null
    else check for first space, return substring after space
}
*/

fn execCmd(input: &str) {
    Command::new(input);
}

enum Builtins {Cd, Echo, Exit, Pwd, Type}

struct Shell {
    prompt: String,
    input: String,
    cmd: Option<String>,
    arg: Option<String>,
    path: Option<String>,
    cwd: Option<String>,
}

impl Shell {
    fn checkBuiltin(){}
    fn printPrompt(&self) {println!("{}", &self.prompt);}
    fn printWorkingDir() {}
    fn handleInput(&self) {
        execCmd(&self.input);
        // let cmd = parseCmd(&self.input);
        // let arg = parseArg(&self.input);
        // if not builtin execCmd(cmd) 
        // else go through builtin
    }
}

fn main() -> Result <(), Error> {
    let mut shell = Shell {
        prompt: String::from("$"),
        input: String::new(),
        cmd: None,
        arg: None,
        path: None,
        cwd: None};
    loop {
        shell.printPrompt();
        io::stdin()
            .read_line(&mut shell.input)
            .expect("input error");
        shell.handleInput();
    } Ok (())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test1() {
    }
}
