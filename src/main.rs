mod context;
mod env;
mod rsh;

use std::string::String;

use crate::context::context as rshell_ctx;
use crate::env::env as rshell_env;
use crate::rsh::rsh as rshell;

use simple_signal::Signal;

/**
 * Entry point. Continuously process lines of input.
 */
fn main() {
    // Intercept SIG_INT and SIG_TERM to prevent terminating the shell accidentally.
    // Type 'exit' to terminate.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], |_signals| ());

    let mut prompt = String::from("> ");

    let mut ctx: rshell_ctx::Context = rshell_ctx::Context::new();

    rshell_env::init_environment(&mut ctx.env);

    if ctx.rl.load_history("rsh_history").is_err() {
        // no previous history;
        ctx.rl.save_history("rsh_history").unwrap();
    }

    loop {
        if rshell::process_line(&mut prompt, &mut ctx) {
            // If we returned true, exit the infinite loop.
            return;
        }
    }
}
