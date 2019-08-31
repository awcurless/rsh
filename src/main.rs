mod env;
mod rsh;

use std::collections::HashMap;
use std::string::String;

use crate::env::env as rshell_env;
use crate::rsh::rsh as rshell;
/**
 * Entry point. Continuously process lines of input.
 */
fn main() {
    let mut prompt = String::from("> ");
    let mut environ: HashMap<String, String> = HashMap::new();

    rshell_env::init_environment(&mut environ);

    loop {
        if rshell::process_line(&mut prompt, &mut environ) {
            // If we returned true, exit the infinite loop.
            return;
        }
    }
}
