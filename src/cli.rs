use clap::{command, Arg, Command, ValueHint};

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn build() -> Command<'static> {
    command!()
        .name("LED Matrix Remote")
        .subcommand_required(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .global(true)
                .help("Still show commands instead of omitting them"),
        )
        .subcommand(
            Command::new("http")
                .about("Read from stdin how the led matrix should look and send it via HTTP")
                .arg(
                    Arg::new("HTTP Server")
                        .short('s')
                        .long("server")
                        .value_hint(ValueHint::Url)
                        .value_name("URL")
                        .takes_value(true)
                        .help("Specify the HTTP Server")
                        .default_value("http://esp-matrix/"),
                ),
        )
        .subcommand(
            Command::new("mqtt")
                .about("Read from stdin how the led matrix should look and send it via MQTT")
                .arg(
                    Arg::new("MQTT Server")
                        .short('b')
                        .long("broker")
                        .value_name("HOST")
                        .value_hint(ValueHint::Hostname)
                        .takes_value(true)
                        .help("Host on which the MQTT Broker is running")
                        .default_value("localhost"),
                )
                .arg(
                    Arg::new("MQTT Port")
                        .short('p')
                        .long("port")
                        .value_name("INT")
                        .value_hint(ValueHint::Other)
                        .takes_value(true)
                        .help("Port on which the MQTT Broker is running")
                        .default_value("1883"),
                )
                .arg(
                    Arg::new("MQTT Base Topic")
                        .short('t')
                        .long("base-topic")
                        .value_name("STRING")
                        .value_hint(ValueHint::Other)
                        .takes_value(true)
                        .help("MQTT Root Topic of the matrix to publish to")
                        .default_value("espMatrix"),
                ),
        )
}

#[test]
fn verify() {
    build().debug_assert();
}
