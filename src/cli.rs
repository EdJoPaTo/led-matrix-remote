use clap::{App, AppSettings, Arg, SubCommand};

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
