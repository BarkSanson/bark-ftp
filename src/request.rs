use crate::command::{Command, CommandType};
use crate::FtpError;

pub struct FtpRequest {
    pub command: CommandType,
    pub arguments: Option<Vec<String>>,
}

impl FtpRequest {
    pub fn from_string(request: String) -> Result<Self, FtpError>{
        let command = request.split_whitespace().next();

        if command.is_none() {
            return Err(FtpError::RequestError("No command found".to_string()));
        } 

        let command = command.unwrap();

        let arguments = request
            .split_whitespace()
            .skip(1)
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>();

        let command = CommandType::from_string(command.to_string());

        Ok(Self {
            command,
            arguments: Some(arguments),
        })
    }
}

impl FtpRequest {
    // Consumes the request and returns a command
    pub fn to_command(self) -> Box<dyn Command> {
        //match self.command {
        //    CommandType::USER => Box::new(UserCommand::new(self.arguments.unwrap())),
        //    CommandType::QUIT => Box::new(QuitCommand),
        //    _ => unimplemented!("Command {} not implemented", command)
        //}

        todo!()
    }
}