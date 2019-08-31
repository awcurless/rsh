pub mod rsh {

    use crate::env::env as rshell_env;
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
        args: &mut Vec<&str>,
        environ: &HashMap<String, String>,
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

        let arguments = rshell_env::resolve_variables(args, environ);

        let output = Command::new(&command)
            .args(arguments)
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

    /**
     * Parse a single line of input. Split the line into a command and arguments.
     * Execute the command, either a build in command or a program on the machine
     * with the given arguments.
     *
     * @return True to terminate.
     */
    pub fn process_line(prompt: &mut String, environ: &mut HashMap<String, String>) -> bool {
        print!("{:}", prompt.to_string());
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous: Option<Child> = None;

        while let Some(command) = commands.next() {
            let mut args: Vec<&str> = command.trim().split_whitespace().collect();

            if command.len() == 0 {
                break;
            }

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
                    rshell_env::env(environ);
                }
                "export" => {
                    rshell_env::parse_environment(&args, environ);
                }
                cmd => {
                    previous =
                        run_command(commands.peek().is_some(), cmd, &mut args, environ, previous);
                }
            };
        }

        if let Some(mut last) = previous {
            last.wait().unwrap();
        }
        return false;
    }
}
