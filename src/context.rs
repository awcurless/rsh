/**
 * Module for interacting the with global shell context.
 */
pub mod context {
    use std::collections::HashMap;
    use std::process::Child;

    /**
     * Global context for the currently active shell instance.
     */
    pub struct Context {
        pub env: HashMap<String, String>,
        pub alias: HashMap<String, String>,
        pub jobs: Vec<Child>,
    }

    impl Context {
        /**
         * Create a new Context object with empty fields.
         */
        pub fn new() -> Context {
            Context {
                env: HashMap::new(),
                alias: HashMap::new(),
                jobs: Vec::new(),
            }
        }
    }
}
