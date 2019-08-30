use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/**
 * Change the working directory. See bash cd.
 * @param args The directory to navigate to.
 * @return The invoked process.
 */
fn cd(args: &Vec<&str>) -> Option<Child> {
    let dir = args[0];
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
    args: &Vec<&str>,
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
            eprintln!("command: {} caused error {}", command, e);
            None
        }
    }
}

fn parse_environment(args: &Vec<&str>, environ: &mut HashMap<String, String>) {
    args.iter().for_each(|arg| {
        if arg.contains("=") {
            let pair: Vec<&str> = arg.split("=").collect();
            environ.insert(pair[0].to_string(), pair[1].to_string());
        }
    });
}

fn env(environ: &HashMap<String, String>) {
    for (key, value) in environ.into_iter() {
        println!("{}={}", key, value);
    }
}

/**
 * Parse a single line of input. Split the line into a command and arguments.
 * Execute the command, either a build in command or a program on the machine
 * with the given arguments.
 *
 * @return True to terminate.
 */
fn process_line(prompt: &mut String, environ: &mut HashMap<String, String>) -> bool {
    print!("{:}", prompt.to_string());
    stdout().flush().unwrap();

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let mut commands = input.trim().split(" | ").peekable();
    let mut previous: Option<Child> = None;

    while let Some(command) = commands.next() {
        let mut args: Vec<&str> = command.trim().split_whitespace().collect();

        let command = args[0];
        args.remove(0);

        match command {
            "cd" => {
                previous = cd(&args);
            }
            "exit" => {
                return true;
            }
            "setprompt" => {
                prompt.clear();
                prompt.push_str(&args[0]);
            }
            "env" => {
                env(environ);
            }
            "export" => {
                parse_environment(&args, environ);
            }
            cmd => {
                previous = run_command(commands.peek().is_some(), cmd, &args, previous);
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
    let mut prompt = String::from("> ");
    let mut environ: HashMap<String, String> = HashMap::new();

    loop {
        if process_line(&mut prompt, &mut environ) {
            // If we returned true, we should exit the infinite loop.
            return;
        }
    }
}
