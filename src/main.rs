use std::io::{self, BufRead};

mod cli;
mod http;
mod mqtt;
mod parse;
mod sender;
mod topic;

fn main() {
    let stdin = io::stdin();
    let sender = cli::get_sender();
    for line in stdin.lock().lines() {
        let line_text = line.expect("failed to read line");

        match parse::parse(&line_text) {
            None => println!("{}", line_text),
            Some(command) => {
                sender.send(command.topic, command.value);
                println!("{}  ✓", line_text);
            }
        }
    }
}
