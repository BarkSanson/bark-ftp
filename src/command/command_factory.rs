use std::collections::HashMap;
use std::io;
use crate::command::Command;

trait CommandFactory {
    // TODO: change io::Error to a custom error
    fn get_command(&self, comm: &str) -> Result<&Box<dyn Command>, io::Error>;
    fn add_command(&mut self, name: impl Into<String>, comm: Box<dyn Command>);
}

pub struct DefaultCommandFactory {
    commands: HashMap<String, Box<dyn Command>>
}

impl DefaultCommandFactory {
    fn new() -> Self {
        Self {
            commands: HashMap::new()
        }
    }

    fn from_hash_set(commands: HashMap<String, Box<dyn Command>>) -> Self {
        Self {
            commands
        }
    }
}

impl CommandFactory for DefaultCommandFactory {
    // TODO: change io::Error to a custom error
    fn get_command(&self, comm: &str) -> Result<&Box<dyn Command>, io::Error> {
        match self.commands.get(comm) {
            Some(comm) => Ok(comm),
            // TODO: Don't panic
            None => panic!()
        }
    }

    fn add_command(&mut self, name: impl Into<String>, comm: Box<dyn Command>) {
        self.commands.insert(name.into(), comm);
    }
}

