# LED Matrix Remote
![Rust](https://github.com/EdJoPaTo/led-matrix-remote/workflows/Rust/badge.svg)

This tool was created to work with [esp-http-neomatrix-text](https://github.com/EdJoPaTo/esp-http-neomatrix-text) and [esp-mqtt-neomatrix-text](https://github.com/EdJoPaTo/esp-mqtt-neomatrix-text).
Each of them is for a LED Matrix with some commands to set what its displaying: `bri`ghtness, `hue`, `sat`uration and `text`.

# Usage

This tools sets them based on the stdin, so you can just pipe in what you want to set:

```sh
echo "bri 20" | led-matrix-remote http
```

You can also write scripts to send stuff over to the LED Matrix:

```sh
#!/bin/sh

# Print "one" in color red, then wait a second
echo "bri 40"
echo "sat 100"
echo "hue 0"
echo "text one"
sleep 1

# Print "two" in color green, then wait a second
echo "hue 120"
echo "text two"
sleep 1

# Print "three" in color blue
echo "hue 240"
echo "text three"
```

```
./script.sh | led-matrix-remote http
```

## Arguments

This Tool currently has two subcommands: `mqtt` and `http`.
Based on your setup you need to use the fitting subcommand.

If you keep the setting of the ESP at default, you can actually use the default values of this tool, so `led-matrix-remote http` will be enough as the server defaults to `http://esp-matrix/` already.

```plaintext
led-matrix-remote-mqtt
Read from stdin how the led matrix should look and send it via MQTT

USAGE:
    led-matrix-remote mqtt [FLAGS] [OPTIONS]

FLAGS:
    -p, --file-persistence    When enabled the MQTT persistence is done via files within the working directory. Enabling this is more
                              reliable.
    -h, --help                Prints help information
    -V, --version             Prints version information

OPTIONS:
    -b, --base-topic <STRING>    MQTT Root Topic of the matrix to publish to [default: espMatrix]
    -q, --qos <INT>              Define the Quality of Service for the MQTT Messages (0, 1 or 2) [default: 2]
    -s, --mqtt-server <URI>      Specify the MQTT Server [default: tcp://localhost:1883]
```

```plaintext
led-matrix-remote-http
Read from stdin how the led matrix should look and send it via HTTP

USAGE:
    led-matrix-remote http [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --server <URI>    Specify the HTTP Server [default: http://esp-matrix/]
```

# Countdown Example

A script for a simple countdown might look like this:

```bash
#!/bin/bash
for value in {10..0}
do
    echo "text $value"
    sleep 1
done

echo "text The End \o/"
```

```sh
./countdown.sh | led-matrix-remote http
```
