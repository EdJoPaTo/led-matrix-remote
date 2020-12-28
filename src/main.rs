use io::Write;
use retry::retry;
use std::io::{self, BufRead};

mod cli;
mod command;
mod http;
mod mqtt;
mod sender;

fn main() {
    let stdin = io::stdin();
    let mut args = cli::get_runtime_arguments();
    for line in stdin.lock().lines() {
        let line_text = line.expect("failed to read line");

        match command::parse(&line_text) {
            None => println!("{}", line_text),
            Some(command) => {
                if args.verbose {
                    print!("{}  ", line_text);
                }

                retry(retry::delay::Fixed::from_millis(20).take(2), || {
                    let result = args.sender.send(&command);

                    if args.verbose {
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
