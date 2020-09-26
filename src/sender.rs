use crate::topic::Topic;

pub trait Sender {
    fn send(&self, topic: Topic, value: &str);
}
