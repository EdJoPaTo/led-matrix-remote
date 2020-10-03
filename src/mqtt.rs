use crate::sender::Sender;
use crate::topic::{get_verb, Topic};
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

pub fn generate_topic(base_topic: &str, topic: Topic) -> String {
    let verb = get_verb(&topic);
    format!("{}/set/{}", base_topic, verb)
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
    fn send(&self, topic: Topic, value: &str) -> Result<(), String> {
        let topic_string = generate_topic(&self.base_topic, topic);
        publish(&self.client, &topic_string, value, self.qos)
            .map_err(|err| format!("failed to send via mqtt: {}", err))
    }
}
