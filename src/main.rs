use std::io::{self, BufRead};

mod cli;
mod http;
mod mqtt;
mod parse;
mod sender;
mod topic;

fn main() {
    let stdin = io::stdin();
    let args = cli::get_runtime_arguments();
    for line in stdin.lock().lines() {
        let line_text = line.expect("failed to read line");

        match parse::parse(&line_text) {
            None => println!("{}", line_text),
            Some(command) => {
                args.sender
                    .send(command.topic, command.value)
                    .expect("failed to send");
                if args.verbose {
                    println!("{}  ✓", line_text);
                }
            }
        }
    }
}
