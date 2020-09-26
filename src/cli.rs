use crate::http::HttpSender;
use crate::mqtt::MqttSender;
use crate::sender::Sender;
use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("LED Matrix Remote")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("http")
            .about("Read from stdin how the led matrix should look and send it via HTTP")
            .arg(Arg::with_name("HTTP Server")
                .short("s")
                .long("server")
                .value_name("URI")
                .takes_value(true)
                .help("Specify the HTTP Server")
                .default_value("http://esp-matrix/")
            )
        )
        .subcommand(SubCommand::with_name("mqtt")
            .about("Read from stdin how the led matrix should look and send it via MQTT")
            .arg(Arg::with_name("MQTT Server")
                .short("s")
                .long("mqtt-server")
                .value_name("URI")
                .takes_value(true)
                .help("Specify the MQTT Server")
                .default_value("tcp://localhost:1883")
            )
            .arg(Arg::with_name("MQTT Base Topic")
                .short("b")
                .long("base-topic")
                .value_name("STRING")
                .takes_value(true)
                .help("MQTT Root Topic of the matrix to publish to")
                .default_value("espMatrix")
            )
            .arg(Arg::with_name("MQTT QoS")
                .short("q")
                .long("qos")
                .value_name("INT")
                .takes_value(true)
                .help("Define the Quality of Service for the MQTT Messages (0, 1 or 2)")
                .default_value("2")
            )
            .arg(Arg::with_name("MQTT File persistence")
                .short("p")
                .long("file-persistence")
                .help("When enabled the MQTT persistence is done via files within the working directory. Enabling this is more reliable.")
            )
        )
}

pub fn get_sender() -> Box<dyn Sender> {
    let matches = build_cli().get_matches();
    if let Some(http_matches) = matches.subcommand_matches("http") {
        let server = http_matches
            .value_of("HTTP Server")
            .expect("HTTP Server could not be read from command line");

        Box::new(HttpSender::new(server))
    } else if let Some(mqtt_matches) = matches.subcommand_matches("mqtt") {
        let mqtt_server = mqtt_matches
            .value_of("MQTT Server")
            .expect("MQTT Server could not be read from command line");

        let mqtt_base_topic = mqtt_matches
            .value_of("MQTT Base Topic")
            .expect("MQTT Base Topic could not be read from command line");

        let mqtt_qos: i32 = mqtt_matches
            .value_of("MQTT QoS")
            .and_then(|s| s.parse::<i32>().ok())
            .expect("MQTT QoS could not be read from command line. Make sure its 0, 1 or 2");

        let mqtt_file_persistence = mqtt_matches.is_present("MQTT File persistence");

        Box::new(MqttSender::new(
            mqtt_server,
            &mqtt_base_topic,
            mqtt_file_persistence,
            mqtt_qos,
        ))
    } else {
        panic!("There has to be a subcommand http or mqtt");
    }
}
