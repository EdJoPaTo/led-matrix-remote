use crate::command::Command;

pub trait Sender {
    fn send(&mut self, command: &Command) -> Result<(), String>;
}
