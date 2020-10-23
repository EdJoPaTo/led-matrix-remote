use crate::command::Command;
use crate::sender::Sender;
use reqwest::blocking::Client;

#[allow(clippy::module_name_repetitions)]
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
    fn send(&self, command: &Command) -> Result<(), String> {
        let url = format!("{}{}", &self.server, command.get_verb());
        let value = command.get_value_string();

        self.client
            .post(&url)
            .body(value)
            .send()
            .map_err(|err| format!("failed to send via http: {}", err))?;

        Ok(())
    }
}
