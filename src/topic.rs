#[derive(Debug, PartialEq)]
pub enum Topic {
    Bri,
    Hue,
    Sat,
    Text,
}

pub fn get_verb(topic: &Topic) -> &'static str {
    match topic {
        Topic::Bri => "bri",
        Topic::Hue => "hue",
        Topic::Sat => "sat",
        Topic::Text => "text",
    }
}

pub fn parse(input: &str) -> Option<Topic> {
    match input.to_lowercase().as_ref() {
        "hue" => Some(Topic::Hue),
        "bri" => Some(Topic::Bri),
        "text" => Some(Topic::Text),
        "sat" => Some(Topic::Sat),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        assert_eq!(Some(Topic::Bri), parse("bri"));
        assert_eq!(Some(Topic::Hue), parse("hue"));
        assert_eq!(Some(Topic::Sat), parse("sat"));
        assert_eq!(Some(Topic::Text), parse("text"));
    }

    #[test]
    fn parse_can_parse_strange_casing() {
        assert_eq!(Some(Topic::Bri), parse("Bri"));
        assert_eq!(Some(Topic::Hue), parse("hUe"));
        assert_eq!(Some(Topic::Sat), parse("saT"));
    }

    #[test]
    fn parse_ignores_wrong() {
        assert_eq!(None, parse("whatever"))
    }
}
