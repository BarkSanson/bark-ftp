use crate::command::Command;

struct QuitCommand;

impl Command for QuitCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("Quit command executed");
        Ok(())
    }
}