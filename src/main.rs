use std::io::{self, BufRead};

mod cli;
mod command;
mod http;
mod mqtt;
mod sender;

fn main() {
    let stdin = io::stdin();
    let args = cli::get_runtime_arguments();
    for line in stdin.lock().lines() {
        let line_text = line.expect("failed to read line");

        match command::parse(&line_text) {
            None => println!("{}", line_text),
            Some(command) => {
                args.sender.send(&command).expect("failed to send");
                if args.verbose {
                    println!("{}  ✓", line_text);
                }
            }
        }
    }
}
