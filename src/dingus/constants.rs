pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub mod common {
    pub const ABOUT: &str = r#"
Manage your computing environments variables with ease!
Inspired by Juan Karam's original Ruby implementation!"#;

    pub const CONFIG_ARG_TEXT: &str =
        "The Yaml file to be read from that contains the necessary \
         enviroment variables. The file must live in `~/.config/dingus/`.";
}

pub mod print {
    pub const ABOUT: &str = "Prints a shell command to standard out.";

    pub const SHELL_ARG_TEXT: &str = "Specify the name of your shell environment.";
}

pub mod session {
    pub const ABOUT: &str = "Open a new shell with environment variables applied.";

    pub const SHELL_ARG_TEXT: &str =
        "Specify the shell program you'd like run after your environment \
         is set up.";
}

pub mod list {
    pub const ABOUT: &str = "List possible options available for --config option.";
}
