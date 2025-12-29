use std::io::Stdin;

struct Shell {
    prompt: String,
    input: String,
}

impl Shell {
    fn printPrompt(&self) {println!("{}", &self.prompt);}
}

fn main() {
    let mut shell = Shell {prompt: String::from("$"), input: String::new()};
    let mut input = String::new();
    while true {
        shell.printPrompt();
        io::stdin()
            .read_line(&mut shell.input)
            .expect("input error");
        println!("invalid command");
    }
}
