use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::str::SplitWhitespace;

/**
 * Change the working directory. See bash cd.
 * @param args The directory to navigate to.
 * @return The invoked process.
 */
fn cd(args: SplitWhitespace) -> Option<Child> {
    let dir = args.peekable().peek().map_or("/", |x| *x);
    println!("folder {}", dir);
    let path = Path::new(dir);

    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("{}", e);
    }

    None
}

/**
 * Executes a program with arguments.
 * @param piped True if this command is part of a piped invocation.
 * @param command The program to execute.
 * @param args Arguments to pass into command.
 * @param previous The previously executed process.
 */
fn run_command(
    piped: bool,
    command: &str,
    args: &mut SplitWhitespace,
    previous: Option<Child>,
) -> Option<Child> {
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
}

/**
 * Parse a single line of input. Split the line into a command and arguments.
 * Execute the command, either a build in command or a program on the machine
 * with the given arguments.
 * @return True to terminate.
 */
fn process_line() -> bool {
    print!("> ");
    stdout().flush().unwrap();

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let mut commands = input.trim().split(" | ").peekable();
    let mut previous: Option<Child> = None;

    while let Some(command) = commands.next() {
        let mut args = command.trim().split_whitespace();

        let command = args.next().unwrap_or("");

        match command {
            "cd" => {
                previous = cd(args);
            }
            "exit" => {
                return true;
            }
            cmd => {
                previous = run_command(commands.peek().is_some(), cmd, &mut args, previous);
            }
        };
    }

    if let Some(mut last) = previous {
        last.wait().unwrap();
    }
    return false;
}

/**
 * Entry point. Continuously process lines of input.
 */
fn main() {
    loop {
        if process_line() {
            // If we returned true, we should exit the infinite loop.
            return;
        }
    }
}
