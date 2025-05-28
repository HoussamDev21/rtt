use std::{fmt, io::Result};

pub struct CommandsHolder {
    pub commands: Vec<Box<dyn Fn() -> Result<()>>>,
}

impl CommandsHolder {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn push<F>(&mut self, command: F)
    where
        F: Fn() -> Result<()> + 'static,
    {
        self.commands.push(Box::new(command));
    }

    pub fn exec_all(&self) {
        for command in &self.commands {
            match command() {
                Ok(_) => {}
                Err(e) => println!("Error executing command: {}", e),
            }
        }
    }
}

impl fmt::Debug for CommandsHolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "[CommandsHolder]")
    }
}
