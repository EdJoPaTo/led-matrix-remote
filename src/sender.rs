use crate::command::Command;

pub trait Sender {
    fn send(&self, command: &Command) -> Result<(), String>;
}
