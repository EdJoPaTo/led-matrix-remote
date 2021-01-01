use crate::command::Command;
use crate::sender::Sender;
use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::thread;

#[allow(clippy::module_name_repetitions)]
pub struct MqttSender {
    base_topic: String,
    client: Client,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl MqttSender {
    pub fn new(host: &str, port: u16, base_topic: &str) -> MqttSender {
        let client_id = format!("led-matrix-remote-{:x}", rand::random::<u32>());
        let mqttoptions = MqttOptions::new(client_id, host, port);
        let (client, connection) = Client::new(mqttoptions, 10);

        let thread_handle = thread::Builder::new()
            .name("mqtt connection".into())
            .spawn(move || thread_logic(connection))
            .expect("failed to start mqtt thread");

        MqttSender {
            client,
            base_topic: base_topic.to_owned(),
            thread_handle: Some(thread_handle),
        }
    }
}

impl Drop for MqttSender {
    fn drop(&mut self) {
        // Try to disconnect and wait but dont care if that doesnt work (-> or default)
        self.client.disconnect().unwrap_or_default();
        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle.join().unwrap_or_default();
        }
    }
}

fn thread_logic(mut connection: Connection) {
    for notification in connection.iter() {
        if let rumqttc::Event::Outgoing(rumqttc::Outgoing::Disconnect) =
            notification.expect("mqtt connection error")
        {
            break;
        }
    }
}

impl Sender for MqttSender {
    fn send(&mut self, command: &Command) -> Result<(), String> {
        let topic = format!("{}/set/{}", &self.base_topic, command.get_verb());
        let payload = command.get_value_string();
        self.client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .map_err(|err| format!("failed to send via mqtt: {}", err))
    }
}
