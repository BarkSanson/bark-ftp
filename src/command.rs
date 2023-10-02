enum Commands {
    USER,
    LIST,
    RETR,
    STOR,
    DELE,
    QUIT
}

trait Command {
    fn is_valid() -> bool;
}

struct QuitCommand;
impl Command for QuitCommand {

}