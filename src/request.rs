use crate::command::CommandType;

pub struct FtpRequest {
    pub command: CommandType,
    arguments: Option<Vec<String>>,
}

impl FtpRequest {
    pub fn from_string(request: String) -> Self {
        let command = request.split_whitespace().next().unwrap();
        let arguments = request
            .split_whitespace()
            .skip(1)
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>();

        let command = CommandType::from_string(command.to_string());

        Self {
            command,
            arguments: Some(arguments),
        }
    }
}