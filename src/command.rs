use clap::{Args, Parser};

/// A command line tool for matching regex expressions.
///
/// Returns an error code if the expression does not match
/// the haystack.
#[derive(Parser, Debug)]
#[command(author = "Yori", version, about, long_about)]
pub struct RegexCommand {
    /// The RegEx pattern to match. Set to "-" to use stdin.
    pub pattern: String,

    /// The string to match. Set to "-" to use stdin.
    ///
    /// If stdin is piped, and the pattern is not taken from
    /// stdin, this argument will be ignored and stdin
    /// will be used as the haystack
    pub haystack: Option<String>,

    /// Ignore case
    #[arg(short = 'i', long)]
    pub case_insensitive: bool,

    #[command(flatten)]
    pub line_endings: LineEndings,

    /// Dot (.) matches newline
    #[arg(short, long = "dotall")]
    pub single_line: bool,

    /// ^/$ match the start and end of a line, respectively
    #[arg(short, long)]
    pub multiline: bool,

    /// Ignore whitespace in the pattern.
    ///
    /// Without this flag, " +" will literally match the whitespace character.
    /// With this flag, the same pattern must be written as "\ +"
    ///
    /// This flag will also configure # to be a comment character, ignoring all
    /// characters until the next line.
    #[arg(short = 'w', long)]
    pub ignore_whitespace: bool,

    /// Swaps greediness.
    ///
    /// Without this flag, + will match as many characters as possible, and
    /// +? will match as few characters as possible. With the flag, these
    /// behaviors are reversed.
    #[arg(short = 'g', long)]
    pub swap_greed: bool,

    /// Enables octal numbers. Defaults to off because that is the default.
    ///
    /// Enabling this feature will disable the error message that signifies
    /// that backreferences are not supported.
    #[arg(short, long)]
    pub octal: bool,

    /// Disable unicode. By default, dot (.) will match unicode once whereas
    /// it would otherwise match more than once (depending on the character).
    /// Disabling this feature
    #[arg(short = 'U', long = "disable-unicode")]
    pub disable_unicode: bool,

    /// Enable verbose output. When enabled, debug information will be printed
    /// to stdout.
    #[arg(short, long)]
    pub verbose: bool,
}

impl RegexCommand {
    /// Prints only if verbosity is enabled
    pub fn debug(&self, info: impl std::fmt::Display) {
        if self.verbose {
            println!("{info}")
        }
    }
    /// Runs a function only if verbose
    pub fn debug_fn(&self, func: impl Fn(&Self)) {
        if self.verbose {
            func(self);
        }
    }
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct LineEndings {
    /// Both \r and \n match line terminators.
    #[arg(short = 'n', long)]
    pub crlf: bool,

    /// Customize line ending; ASCII only
    #[arg(short, long)]
    pub line_ending: Option<char>,
}
