use clap::{app_from_crate, App, AppSettings, Arg};

#[must_use]
pub fn build() -> App<'static> {
    app_from_crate!()
        .name("LED Matrix Remote")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .global(true)
                .about("Still show commands instead of omitting them"),
        )
        .subcommand(
            App::new("http")
                .about("Read from stdin how the led matrix should look and send it via HTTP")
                .arg(
                    Arg::new("HTTP Server")
                        .short('s')
                        .long("server")
                        .value_name("URI")
                        .takes_value(true)
                        .about("Specify the HTTP Server")
                        .default_value("http://esp-matrix/"),
                ),
        )
        .subcommand(
            App::new("mqtt")
                .about("Read from stdin how the led matrix should look and send it via MQTT")
                .arg(
                    Arg::new("MQTT Server")
                        .short('b')
                        .long("broker")
                        .value_name("HOST")
                        .takes_value(true)
                        .about("Host on which the MQTT Broker is running")
                        .default_value("localhost"),
                )
                .arg(
                    Arg::new("MQTT Port")
                        .short('p')
                        .long("port")
                        .value_name("INT")
                        .takes_value(true)
                        .about("Port on which the MQTT Broker is running")
                        .default_value("1883"),
                )
                .arg(
                    Arg::new("MQTT Base Topic")
                        .short('t')
                        .long("base-topic")
                        .value_name("STRING")
                        .takes_value(true)
                        .about("MQTT Root Topic of the matrix to publish to")
                        .default_value("espMatrix"),
                ),
        )
}
