pub mod env {

    use std::collections::HashMap;
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
     * @param start Position of the start of the variable inside the argument.
     * @param stop Position of the end of the variable inside the argument.
     * @param arg The argument or command to process.
     * @return The variable to be looked up.
     */
    fn parse_variable(start: &mut usize, stop: &mut usize, arg: &String) -> String {
        let mut var = String::new();
        for i in 0..arg.len() {
            let c = arg.chars().nth(i);
            if c == Some('$') {
                *start = i;
            } else if *stop == 0 {
                let c = c.unwrap();
                if c == ' ' {
                    *stop = i;
                }
                if c == '$' {
                    // This is the case where there are 2 or more variables in the same argument.
                    // This is not yet supported.
                    *stop = i;
                    break;
                } else {
                    var.push(c);
                }
            }
        }
        if *stop == 0 {
            *stop = arg.len();
        }
        var
    }

    /**
     * Given a vector of arguments, resolve variables in each argument and return a vector with the
     * evaluated arguments.
     * @param args Vector arguments.
     * @param environ Hashmap containing environment.
     * @return Vector of fully-parsed arguments.
     */
    pub fn resolve_variables(args: &Vec<&str>, environ: &HashMap<String, String>) -> Vec<String> {
        let mut evaluated: Vec<String> = Vec::new();
        for arg in args {
            let mut arg = arg.to_string();
            if arg.contains("$") {
                let mut start = 0;
                let mut end = 0;

                let var = parse_variable(&mut start, &mut end, &mut arg);
                let key: Option<&String> = environ.get(&var);

                match key {
                    Some(k) => {
                        arg.replace_range(start..end, k.as_ref());
                        evaluated.push(arg);
                    }
                    None => println!("Unknown variable: {}", var),
                }
            } else {
                evaluated.push(arg);
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
