use io::Write;
use retry::retry;
use std::io::{self, BufRead};

use crate::http::HttpSender;
use crate::mqtt::MqttSender;
use crate::sender::Sender;

mod cli;
mod command;
mod http;
mod mqtt;
mod sender;

fn main() {
    let stdin = io::stdin();

    let matches = cli::build().get_matches();
    let verbose = matches.is_present("verbose");
    let mut sender: Box<dyn Sender> = match matches.subcommand().expect("expected a subcommand") {
        ("http", http_matches) => {
            let server = http_matches
                .value_of("HTTP Server")
                .expect("HTTP Server could not be read from command line");

            Box::new(HttpSender::new(server))
        }
        ("mqtt", mqtt_matches) => {
            let mqtt_host = mqtt_matches
                .value_of("MQTT Server")
                .expect("MQTT Host could not be read from command line");

            let mqtt_port = mqtt_matches
                .value_of("MQTT Port")
                .and_then(|s| s.parse::<u16>().ok())
                .expect("MQTT Port could not be read from command line");

            let mqtt_base_topic = mqtt_matches
                .value_of("MQTT Base Topic")
                .expect("MQTT Base Topic could not be read from command line");

            Box::new(MqttSender::new(mqtt_host, mqtt_port, mqtt_base_topic))
        }
        _ => unimplemented!("Unknown subcommand"),
    };

    for line in stdin.lock().lines() {
        let line_text = line.expect("failed to read line");

        match command::parse(&line_text) {
            None => println!("{}", line_text),
            Some(command) => {
                if verbose {
                    print!("{}  ", line_text);
                }

                retry(retry::delay::Fixed::from_millis(20).take(2), || {
                    let result = sender.send(&command);

                    if verbose {
                        #[allow(clippy::non_ascii_literal)]
                        if result.is_ok() {
                            println!("✓");
                        } else {
                            print!("✗");
                            io::stdout().flush().expect("failed to flush stdout");
                        }
                    }

                    result
                })
                .expect("failed to send");
            }
        }
    }
}
