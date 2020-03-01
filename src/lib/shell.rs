use super::error::Error;
use std::env::var;

#[derive(Debug)]
pub struct Shell {
    command: String,
}
impl Shell {
    pub fn shell_command(&self) -> &str {
        &self.command
    }

    pub fn current_shell() -> Result<Self, Error> {
        let command = var("SHELL")?;
        Ok(Self { command })
    }
}
