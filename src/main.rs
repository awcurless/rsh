pub mod rsh;

pub use crate::rsh::rsh as rshell;

use std::collections::HashMap;
use std::string::String;

/**
 * Entry point. Continuously process lines of input.
 */
fn main() {
    let mut prompt = String::from("> ");
    let mut environ: HashMap<String, String> = HashMap::new();

    loop {
        if rshell::process_line(&mut prompt, &mut environ) {
            // If we returned true, we should exit the infinite loop.
            return;
        }
    }
}
