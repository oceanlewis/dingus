pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub mod common {
    pub const ABOUT: &'static str = r#"
Manage your computing environments variables with ease!

By default `dingus` will believe your current shell is whatever program your
$SHELL evironment variable is set to. I would not recommend changing this,
instead you can tell `dingus` to use a different shell by supplying a
`--shell SHELL` argument.

Inspired by Juan Karam's original Ruby implementation!"#;

    pub const CONFIG_ARG_TEXT: &'static str =
        "The Yaml file to be read from that contains the necessary \
         enviroment variables. The file must live in `~/.config/dingus/`.";
}

pub mod print {
    pub const ABOUT: &'static str =
        "Print out a shell command you can run to apply variables directly \
         to your current session.";

    pub const SHELL_ARG_TEXT: &'static str = "Specify the name of your shell environment.";
}

pub mod session {
    pub const ABOUT: &'static str =
        r#"Open a new shell with environment variables applied. Changes made to
that session will not affect the parent session."#;

    pub const SHELL_ARG_TEXT: &'static str =
        "Specify the shell program you'd like run after your environment \
         is set up.";
}

pub mod list {
    pub const ABOUT: &'static str = "List possible options available for --config option.";
}
