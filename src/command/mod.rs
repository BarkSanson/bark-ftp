mod command_factory;

pub trait Command {
    fn execute(&self);
}
