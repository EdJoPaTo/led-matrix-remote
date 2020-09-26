use crate::topic::Topic;

#[derive(Debug, PartialEq)]
pub struct Command<'a> {
    pub topic: Topic,
    pub value: &'a str,
}

pub fn parse(input: &str) -> Option<Command<'_>> {
    let trimmed = input.trim();

    if let Some(splitter) = trimmed.find(' ') {
        if let Some(topic) = trimmed.get(..splitter).and_then(crate::topic::parse) {
            if let Some(value) = trimmed.get(splitter + 1..) {
                return Some(Command { topic, value });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ignores_comments() {
        assert_eq!(None, parse("# some comment"));
    }
    #[test]
    fn parse_works() {
        match parse("bri 60") {
            Some(result) => {
                assert_eq!(result.topic, Topic::Bri);
                assert_eq!(result.value, "60");
            }
            None => panic!("should work"),
        }
    }

    #[test]
    fn parse_parses_text() {
        match parse("text stuff") {
            Some(result) => {
                assert_eq!(result.topic, Topic::Text);
                assert_eq!(result.value, "stuff");
            }
            None => panic!("should work"),
        }
    }

    #[test]
    fn parse_parses_text_with_spaces() {
        match parse("text stuff with spaces") {
            Some(result) => {
                assert_eq!(result.topic, Topic::Text);
                assert_eq!(result.value, "stuff with spaces");
            }
            None => panic!("should work"),
        }
    }
}
