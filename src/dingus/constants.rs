pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub mod common {
    pub const ABOUT: &'static str = r#"
Manage your computing environments variables with ease!
Inspired by Juan Karam's original Ruby implementation!"#;

    pub const CONFIG_ARG_TEXT: &'static str =
        "The Yaml file to be read from that contains the necessary \
         enviroment variables. The file must live in `~/.config/dingus/`.";
}

pub mod print {
    pub const ABOUT: &'static str = "Prints a shell command to standard out.";

    pub const SHELL_ARG_TEXT: &'static str = "Specify the name of your shell environment.";
}

pub mod session {
    pub const ABOUT: &'static str = "Open a new shell with environment variables applied.";

    pub const SHELL_ARG_TEXT: &'static str =
        "Specify the shell program you'd like run after your environment \
         is set up.";
}

pub mod list {
    pub const ABOUT: &'static str = "List possible options available for --config option.";
}
