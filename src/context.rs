/**
 * Module for interacting the with global shell context.
 */
pub mod context {
    use std::borrow::Cow::{self, Borrowed, Owned};
    use std::collections::HashMap;
    use std::process::Child;

    use rustyline::completion::{Completer, FilenameCompleter, Pair};
    use rustyline::config::OutputStreamType;
    use rustyline::error::ReadlineError;
    use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
    use rustyline::hint::{Hinter, HistoryHinter};
    use rustyline::{CompletionType, Config, EditMode, Editor};
    use rustyline_derive::Helper;

    #[derive(Helper)]
    pub struct RshHelper {
        completer: FilenameCompleter,
        highlighter: MatchingBracketHighlighter,
        hinter: HistoryHinter,
    }

    impl Completer for RshHelper {
        type Candidate = Pair;

        fn complete(
            &self,
            line: &str,
            pos: usize,
            ctx: &rustyline::Context<'_>,
        ) -> Result<(usize, Vec<Pair>), ReadlineError> {
            self.completer.complete(line, pos, ctx)
        }
    }

    impl Hinter for RshHelper {
        fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<String> {
            self.hinter.hint(line, pos, ctx)
        }
    }

    impl Highlighter for RshHelper {
        fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
            &'s self,
            prompt: &'p str,
            _default: bool,
        ) -> Cow<'b, str> {
            Borrowed(prompt)
        }

        fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
            Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
        }

        fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
            self.highlighter.highlight(line, pos)
        }

        fn highlight_char(&self, line: &str, pos: usize) -> bool {
            self.highlighter.highlight_char(line, pos)
        }
    }

    /**
     * Global context for the currently active shell instance.
     */
    pub struct Context {
        pub env: HashMap<String, String>,
        pub alias: HashMap<String, String>,
        pub jobs: Vec<Child>,
        pub rl: Editor<RshHelper>,
    }

    impl Context {
        /**
         * Create a new Context object with empty fields.
         */
        pub fn new() -> Context {
            let rustyline_cfg = Config::builder()
                .history_ignore_space(true)
                .history_ignore_dups(true)
                .completion_type(CompletionType::List)
                .edit_mode(EditMode::Vi)
                .output_stream(OutputStreamType::Stdout)
                .build();
            let helper = RshHelper {
                completer: FilenameCompleter::new(),
                highlighter: MatchingBracketHighlighter::new(),
                hinter: HistoryHinter {},
            };
            let mut context = Context {
                env: HashMap::new(),
                alias: HashMap::new(),
                jobs: Vec::new(),
                rl: Editor::with_config(rustyline_cfg),
            };

            context.rl.set_helper(Some(helper));
            return context;
        }
    }
}
