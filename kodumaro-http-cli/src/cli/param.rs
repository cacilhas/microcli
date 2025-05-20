use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    sync::LazyLock,
};

use eyre::eyre;
use regex::Regex;
use reqwest::Url;
use serde_json::{Map, Value};

use super::util::parse_string;


static FILE_PAYLOAD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::from_str(r#"^@.+$"#).unwrap());
static HEADER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::from_str(r#"^([\w-]+):(.*)$"#).unwrap());
static QUERY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::from_str(r#"^([^=:]+)==(.*)$"#).unwrap());
static PAYLOAD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::from_str(r#"^([^=:]+)=(.*)$"#).unwrap());


#[derive(Clone, Debug, PartialEq)]
pub enum Param {
    Header(String, String),
    Payload(Value),
    Query(String, String),
}

impl Display for Param {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Header(name, value) => write!(f, "{}:{}", name, value),
            Self::Payload(value) => write!(f, "{}", value),
            Self::Query(key, value) => {
                let mut buf = Url::parse("http://localhost/").unwrap();
                buf.query_pairs_mut().append_pair(key, value);
                write!(f, "{}", buf.query().unwrap())
            }
        }
    }
}

impl FromStr for Param {

    type Err = eyre::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {

        if FILE_PAYLOAD_REGEX.is_match(value) {
            let payload: Value = serde_json::from_str(&parse_string(value)?)?;
            return Ok(Self::Payload(payload));
        }

        if let Some(pair) = QUERY_REGEX.captures(value) {
            let key = pair.get(1).ok_or(eyre!("invalid query {}", value))?.as_str();
            let value = pair.get(2).ok_or(eyre!("invalid query {}", value))?.as_str();
            return Ok(Self::Query(key.to_owned(), parse_string(value)?));
        }

        if let Some(pair) = PAYLOAD_REGEX.captures(value) {
            let key = pair.get(1).ok_or(eyre!("invalid attribute {}", value))?.as_str();
            let value = pair.get(2).ok_or(eyre!("invalid attribute {}", value))?.as_str();
            let value = parse_string(value)?;
            let value: Value = string_to_value(&value);
            let mut payload = Map::new();
            payload.insert(key.to_owned(), value);
            return Ok(Self::Payload(Value::Object(payload)))
        }

        if let Some(pair) = HEADER_REGEX.captures(value) {
            let key = pair.get(1)
                .ok_or(eyre!("invalid header {}", value))?
                .as_str()
                .trim();
            let value = pair.get(2)
                .ok_or(eyre!("invalid header {}", value))?
                .as_str()
                .trim();
            return Ok(Self::Header(key.to_owned(), parse_string(value)?));
        }

        Err(eyre!("could not parse {}", value))
    }
}


/*----------------------------------------------------------------------------*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        let param = Param::from_str("Content-Type: application/json").unwrap();
        assert_eq!(Param::Header("Content-Type".to_string(), "application/json".to_string()), param);
    }

    #[test]
    fn test_parse_query() {
        let param = Param::from_str("foo==bar").unwrap();
        assert_eq!(Param::Query("foo".to_string(), "bar".to_string()), param);
    }

    #[test]
    fn test_parse_payload() {
        let param = Param::from_str("num=42").unwrap();
        if let Param::Payload(Value::Object(param)) = param {
            let num= param.get("num").unwrap();
            assert!(num.is_i64());
            assert_eq!(42, num.as_i64().unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn test_invalid_param() {
        let param = Param::from_str("invalid param");
        assert!(param.is_err());
    }
}

fn string_to_value(value: &str) -> Value {
    if value.starts_with("str!") {
        return Value::String(value[4..].to_string());
    }

    serde_json::from_str(&value).unwrap_or(Value::String(value.to_string()))
}
