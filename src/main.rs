use std::io;
use std::process;

struct Shell {
    prompt: String,
    input: String,
    // string optional for command 
    // string optional vec for args
    // builtins? 
}

impl Shell {
    fn printPrompt(&self) {println!("{}", &self.prompt);}
    // parse cmds
    // parse args
    // check builtin
    fn handleInput(&self) {
        let _in = &self.input;
        if (_in.contains("exit")) {
            process::exit(0);
        } else {print!("invalid command");}
    }
}

fn main() {
    let mut shell = Shell {prompt: String::from("$"), input: String::new()};
    loop {
        shell.printPrompt();
        io::stdin()
            .read_line(&mut shell.input)
            .expect("input error");
        shell.handleInput();
    }
}
