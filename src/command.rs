#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    On(bool),
    Bri(u8),
    Hue(u16),
    Sat(u8),
    Text(&'a str),
}

impl Command<'_> {
    pub fn get_verb(&self) -> &'static str {
        match &self {
            Command::On(_) => "on",
            Command::Bri(_) => "bri",
            Command::Hue(_) => "hue",
            Command::Sat(_) => "sat",
            Command::Text(_) => "text",
        }
    }

    pub fn get_value_string(&self) -> String {
        match &self {
            Command::On(true) => "1".to_string(),
            Command::On(false) => "0".to_string(),
            Command::Bri(value) | Command::Sat(value) => format!("{}", value),
            Command::Hue(value) => format!("{}", value),
            Command::Text(value) => (*value).to_string(),
        }
    }
}

pub fn parse(input: &str) -> Option<Command<'_>> {
    let trimmed = input.trim();

    let splitter = trimmed.find(' ')?;
    let topic = trimmed.get(..splitter)?;
    let value_string = trimmed.get(splitter + 1..)?;

    match topic.to_lowercase().as_ref() {
        "on" => match value_string {
            "1" | "true" => Some(Command::On(true)),
            "0" | "false" => Some(Command::On(false)),
            _ => None,
        },
        "hue" => {
            let value = value_string.parse::<u16>().ok()?;
            Some(Command::Hue(value))
        }
        "sat" => {
            let value = value_string.parse::<u8>().ok()?;
            Some(Command::Sat(value))
        }
        "bri" => {
            let value = value_string.parse::<u8>().ok()?;
            Some(Command::Bri(value))
        }
        "text" => Some(Command::Text(value_string)),
        _ => None,
    }
}

#[test]
fn parse_ignores_comments() {
    assert_eq!(None, parse("# some comment"));
}
#[test]
fn parse_works() {
    assert_eq!(Some(Command::Hue(300)), parse("hue 300"));
    assert_eq!(Some(Command::Sat(80)), parse("sat 80"));
    assert_eq!(Some(Command::Bri(60)), parse("bri 60"));
}

#[test]
fn parse_works_with_strange_casing() {
    assert_eq!(Some(Command::Hue(10)), parse("Hue 10"));
    assert_eq!(Some(Command::Sat(10)), parse("sAt 10"));
    assert_eq!(Some(Command::Bri(10)), parse("brI 10"));
}

#[test]
fn parse_parses_text() {
    assert_eq!(Some(Command::Text("stuff")), parse("text stuff"));
}

#[test]
fn parse_parses_on() {
    assert_eq!(Some(Command::On(true)), parse("on true"));
    assert_eq!(Some(Command::On(true)), parse("on 1"));
    assert_eq!(Some(Command::On(false)), parse("on false"));
    assert_eq!(Some(Command::On(false)), parse("on 0"));
}

#[test]
fn parse_parses_text_with_spaces() {
    assert_eq!(
        Some(Command::Text("stuff with spaces")),
        parse("text stuff with spaces")
    );
}

#[test]
fn parse_ignores_wrong() {
    assert_eq!(None, parse("whatever"));
    assert_eq!(None, parse("whatever with spaces"));
}
