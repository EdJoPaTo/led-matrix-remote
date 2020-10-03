use crate::sender::Sender;
use crate::topic::{get_verb, Topic};
use reqwest::blocking::Client;

pub fn generate_url(server: &str, topic: Topic) -> String {
    let verb = get_verb(&topic);
    format!("{}{}", server, verb)
}

pub struct HttpSender {
    server: String,
    client: Client,
}

impl HttpSender {
    pub fn new(server: &str) -> HttpSender {
        let client = Client::new();

        assert!(server.starts_with("http"));
        assert!(server.ends_with('/'));

        HttpSender {
            client,
            server: server.to_owned(),
        }
    }
}

impl Sender for HttpSender {
    fn send(&self, topic: Topic, value: &str) -> Result<(), String> {
        let url = generate_url(&self.server, topic);

        self.client
            .post(&url)
            .body(value.to_owned())
            .send()
            .map_err(|err| format!("failed to send via http: {}", err))?;

        Ok(())
    }
}
