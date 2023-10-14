
pub enum CommandType {
    User,
    Quit,
    Unimplemented,
}

impl CommandType {
    pub fn from_string(command: String) -> Self {
        match command.as_str() {
            "USER" => Self::User,
            "QUIT" => Self::Quit,
            _ => Self::Unimplemented,
        }
    }
}

pub trait Command {
    fn execute(&self);
}
