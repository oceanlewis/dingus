use ansi_term::Style;
use structopt::StructOpt;

mod lib;
use lib::cli::Cli;

fn main() {
    if let Err(e) = Cli::from_args().run() {
        eprintln!("{} {}", Style::new().bold().paint("Error:"), e);
    }
}
