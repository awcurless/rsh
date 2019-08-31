pub mod env {

    use std::collections::HashMap;
    use std::ops::Range;
    use std::process::Command;
    use std::str;

    /**
     * Populate the environment map with default values.
     * TODO: read /etc/environment, profile and other locations to get variables.
     * @param environ Map to store variables in.
     */
    pub fn init_environment(environ: &mut HashMap<String, String>) {
        environ.insert(
            String::from("USER"),
            str::from_utf8(
                Command::new("id")
                    .arg("-un")
                    .output()
                    .unwrap()
                    .stdout
                    .as_slice(),
            )
            .unwrap()
            .to_string(),
        );
    }

    /**
     * Parse an arugment or command, resolving defined variable references.
     *
     * @param arg The argument or command to process.
     * @param start Position of the start of the variable inside the argument.
     * @param ranges Index ranges in which variables appear.
     */
    fn parse_variable(arg: &str, start: usize, ranges: &mut Vec<String>) {
        let mut beginning = 0;
        let mut stop = start;

        for i in start..arg.len() {
            let c = arg.chars().nth(i).unwrap();

            if c == '$' && start == i {
                beginning = i;
            } else {
                if c == ' ' || c == '\n' {
                    stop = i;
                } else if c == '$' {
                    stop = i;
                    parse_variable(arg, i, ranges);
                }
            }
        }
        if stop == start {
            stop = arg.len();
        }
        ranges.push(arg[beginning..stop].trim().to_string());
    }

    /**
     * Given a vector of arguments, resolve variables in each argument and return a vector with the
     * evaluated arguments.
     * @param args Vector arguments.
     * @param environ Hashmap containing environment.
     * @return Vector of fully-parsed arguments.
     */
    pub fn resolve_variables(
        args: &mut Vec<&str>,
        environ: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut evaluated: Vec<String> = Vec::new();
        for arg in args {
            if arg.contains("$") {
                let mut arg = arg.clone();
                let mut ranges: Vec<String> = Vec::new();
                parse_variable(&mut arg, 0, &mut ranges);

                ranges.reverse();
                for range in ranges {
                    let value: Option<&String> = environ.get(&range[1..]);

                    match value {
                        Some(v) => {
                            let eval = String::from(arg.clone());
                            let eval = eval.replace(&range, v.as_ref());
                            evaluated.push(eval);
                        }
                        None => println!("Unknown variable: {:?}", &range),
                    }
                }
            } else {
                evaluated.push(String::from(arg.clone()));
            }
        }
        evaluated
    }

    /**
     * Dump environment variables to the console.
     * @param environ Hashmap containing variables.
     */
    pub fn env(environ: &HashMap<String, String>) {
        for (key, value) in environ.into_iter() {
            println!("{}={}", key, value);
        }
    }

    /**
     * Parse a vector of strings in "key=value" format into the given hashmap.
     * @param args Arguments to parse into environment variables.
     * @param environ Hashmap to store variables in.
     */
    pub fn parse_environment(args: &Vec<&str>, environ: &mut HashMap<String, String>) {
        args.iter().for_each(|arg| {
            if arg.contains("=") {
                let pair: Vec<&str> = arg.split("=").collect();
                environ.insert(pair[0].to_string(), pair[1].to_string());
            }
        });
    }
}
