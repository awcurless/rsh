use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::str::SplitWhitespace;

fn cd(args: SplitWhitespace) -> Option<Child> {
    let dir = args.peekable().peek().map_or("/", |x| *x);
    println!("folder {}", dir);
    let path = Path::new(dir);

    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("{}", e);
    }

    None
}

fn run_command(piped: bool, args: &mut SplitWhitespace, previous: Option<Child>) -> Option<Child> {
    if let Some(command) = args.next() {
        let stdin = previous.map_or(Stdio::inherit(), |output: Child| {
            Stdio::from(output.stdout.unwrap())
        });

        let stdout = if piped {
            Stdio::piped()
        } else {
            Stdio::inherit()
        };

        let output = Command::new(command)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn();

        match output {
            Ok(output) => Some(output),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    } else {
        None
    }
}

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous: Option<Child> = None;

        while let Some(command) = commands.next() {
            let mut args = command.trim().split_whitespace();

            match command {
                "cd" => {
                    previous = cd(args);
                }
                "exit" => {
                    return;
                }
                _ => {
                    previous = run_command(commands.peek().is_some(), &mut args, previous);
                }
            };
        }

        if let Some(mut last) = previous {
            last.wait().unwrap();
        }
    }
}
