mod quit;

pub enum CommandType {
    USER,
    QUIT,
    UNIMPLEMENTED,
}

impl CommandType {
    pub fn from_string(command: String) -> Self {
        match command.as_str() {
            "USER" => Self::USER,
            "QUIT" => Self::QUIT,
            _ => Self::UNIMPLEMENTED,
        }
    }
}

pub trait Command {
    fn execute(&self);
}
