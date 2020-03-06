pub mod rsh {

    use crate::context::context::Context;
    use crate::env::env as rshell_env;
    use std::collections::HashMap;
    use std::env;
    use std::path::Path;
    use std::process::{Child, Command, ExitStatus, Stdio};

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
    pub fn process_line(prompt: &mut String, ctx: &mut Context) -> bool {
        let input = match ctx.rl.readline(prompt) {
            Ok(line) => line,
            Err(_) => String::from(""),
        };

        ctx.rl.add_history_entry(input.as_str());
        let fixed;
        let background;
        if input.contains('&') {
            background = input.chars().nth(input.len() - 2).unwrap() == '&';
            fixed = match background {
                true => String::from(&input[0..input.len() - 3]),
                false => input,
            };
        } else {
            fixed = input;
            background = false;
        }

        let mut commands = fixed.trim().split(" | ").peekable();
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
                    ctx.rl.save_history("rsh_history").unwrap();
                    return true;
                }
                "setprompt" => {
                    prompt.clear();
                    prompt.push_str(&args[0]);
                }
                "env" => {
                    rshell_env::env(&ctx.env);
                }
                "export" => {
                    rshell_env::parse_environment(&args, &mut ctx.env);
                }
                "jobs" => {
                    jobs(&ctx.jobs);
                }
                cmd => {
                    previous = run_command(
                        commands.peek().is_some(),
                        cmd,
                        &mut args,
                        &ctx.env,
                        previous,
                    );
                }
            };
        }

        if let Some(mut last) = previous {
            // Handle synchronous and asynchronous jobs.
            if !background {
                last.wait().unwrap();
            } else {
                match last.try_wait() {
                    Ok(Some(_status)) => (),
                    Ok(None) => {
                        // Still running so we need to store it.
                        println!("[{}]", &last.id());
                        ctx.jobs.push(last);
                    }
                    Err(e) => eprintln!("Error collecting background process state {:?}", e),
                };
            }
        }

        check_running_jobs(&mut ctx.jobs);

        return false;
    }

    /**
     * List the currently active jobs to stdout.
     *
     * Format:
     * [<id>] <pid>
     *
     * where <id> is a shell assigned index of the job and <pid> is the OS assigned
     * process ID.
     *
     * @param jobs List of active jobs maintained in Context.
     */
    fn jobs(jobs: &Vec<Child>) {
        let mut idx: usize = 1;
        jobs.iter().for_each(|j| {
            println!("[{}] {}", idx, j.id());
            idx += 1;
        });
    }

    /**
     * Loop the jobs listing and check to see if each job is still in process.
     * Delete finished jobs from the list.
     * @param jobs Background job listing maintained in Context.
     */
    fn check_running_jobs(jobs: &mut Vec<Child>) {
        let mut completed_pids: Vec<u32> = Vec::new();

        let mut idx: usize = 1;
        for job in &mut *jobs {
            match check_process(job) {
                Some(_status) => {
                    match job.stdout.as_ref() {
                        Some(output) => print!("{:?}", output),
                        None => (),
                    }
                    println!("[{}] {} done", idx, job.id());
                    completed_pids.push(job.id());
                }
                None => (),
            }
            idx += 1;
        }

        completed_pids
            .iter()
            .for_each(|pid| jobs.retain(|j| j.id() != *pid));
    }

    /**
     * Checks the state of a given background process.
     * @param job The process.
     * @return The ExitStatus if a process is finished, otherwise None.
     */
    fn check_process(job: &mut Child) -> Option<ExitStatus> {
        match job.try_wait() {
            Ok(Some(status)) => Some(status),
            Ok(None) => None,
            Err(e) => {
                eprintln!("Error collecting background process state {:?}", e);
                None
            }
        }
    }
}
