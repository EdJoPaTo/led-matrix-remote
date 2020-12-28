use crate::http::HttpSender;
use crate::mqtt::MqttSender;
use crate::sender::Sender;
use clap::{App, AppSettings, Arg, SubCommand};

pub struct RuntimeArguments {
    pub verbose: bool,
    pub sender: Box<dyn Sender>,
}

pub fn build() -> App<'static, 'static> {
    App::new("LED Matrix Remote")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .global(true)
                .help("Still show commands instead of omitting them"),
        )
        .subcommand(
            SubCommand::with_name("http")
                .about("Read from stdin how the led matrix should look and send it via HTTP")
                .arg(
                    Arg::with_name("HTTP Server")
                        .short("s")
                        .long("server")
                        .value_name("URI")
                        .takes_value(true)
                        .help("Specify the HTTP Server")
                        .default_value("http://esp-matrix/"),
                ),
        )
        .subcommand(
            SubCommand::with_name("mqtt")
                .about("Read from stdin how the led matrix should look and send it via MQTT")
                .arg(
                    Arg::with_name("MQTT Server")
                        .short("h")
                        .long("host")
                        .value_name("HOST")
                        .takes_value(true)
                        .help("Host on which the MQTT Broker is running")
                        .default_value("localhost"),
                )
                .arg(
                    Arg::with_name("MQTT Port")
                        .short("p")
                        .long("port")
                        .value_name("INT")
                        .takes_value(true)
                        .help("Port on which the MQTT Broker is running")
                        .default_value("1883"),
                )
                .arg(
                    Arg::with_name("MQTT Base Topic")
                        .short("t")
                        .long("base-topic")
                        .value_name("STRING")
                        .takes_value(true)
                        .help("MQTT Root Topic of the matrix to publish to")
                        .default_value("espMatrix"),
                ),
        )
}

pub fn get_runtime_arguments() -> RuntimeArguments {
    let matches = build().get_matches();

    let verbose = matches.is_present("verbose");

    let sender: Box<dyn Sender> = if let Some(http_matches) = matches.subcommand_matches("http") {
        let server = http_matches
            .value_of("HTTP Server")
            .expect("HTTP Server could not be read from command line");

        Box::new(HttpSender::new(server))
    } else if let Some(mqtt_matches) = matches.subcommand_matches("mqtt") {
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

        Box::new(MqttSender::new(mqtt_host, mqtt_port, &mqtt_base_topic))
    } else {
        panic!("There has to be a subcommand http or mqtt");
    };

    RuntimeArguments { sender, verbose }
}
