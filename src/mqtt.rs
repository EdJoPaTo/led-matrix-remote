use crate::command::Command;
use crate::sender::Sender;
use paho_mqtt::{
    Client, ConnectOptionsBuilder, CreateOptionsBuilder, MessageBuilder, MqttError, PersistenceType,
};
use std::time::Duration;

pub fn connect(mqtt_server: &str, file_persistence: bool) -> Result<Client, MqttError> {
    let create_options = CreateOptionsBuilder::new()
        .server_uri(mqtt_server)
        .persistence(if file_persistence {
            PersistenceType::File
        } else {
            PersistenceType::None
        })
        .finalize();

    let client = Client::new(create_options)?;

    let connection_options = ConnectOptionsBuilder::new()
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
        .finalize();

    client.connect(connection_options)?;

    Ok(client)
}

pub fn publish(client: &Client, topic: &str, payload: &str, qos: i32) -> Result<(), MqttError> {
    let msg = MessageBuilder::new()
        .topic(topic)
        .qos(qos)
        .payload(payload)
        .finalize();

    client.publish(msg)
}

pub struct MqttSender {
    base_topic: String,
    qos: i32,
    client: Client,
}

impl MqttSender {
    pub fn new(server: &str, base_topic: &str, file_persistence: bool, qos: i32) -> MqttSender {
        let client = connect(server, file_persistence).expect("failed to connect to MQTT server");

        MqttSender {
            client,
            qos,
            base_topic: base_topic.to_owned(),
        }
    }
}

impl Sender for MqttSender {
    fn send(&self, command: &Command) -> Result<(), String> {
        let topic = format!("{}/set/{}", &self.base_topic, command.get_verb());
        let payload = command.get_value_string();
        publish(&self.client, &topic, &payload, self.qos)
            .map_err(|err| format!("failed to send via mqtt: {}", err))
    }
}
