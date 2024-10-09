mod cli_bool;
mod param;
mod util;

use std::{env::consts, str::FromStr};

use base64::{engine, Engine};
use clap::{ArgAction, Parser, Subcommand};
use cli_bool::CliBool;
use eyre::eyre;
pub use param::Param;
use reqwest::{header::{HeaderName, HeaderValue}, redirect::Policy, Method, Request, Url};
use serde_json::Value;
use util::parse_string;

#[derive(Debug, Parser)]
#[command(about, author, name = "http", version)]
pub struct Cli {

    #[command(subcommand)]
    pub verb: Verb,

    // /// data items from the command line are serialized as a JSON object
    // #[arg(short, long, action = ArgAction::SetTrue, default_value_t = true)]
    // pub json: bool,

    // /// data items from the command line are serialized as form fields
    // #[arg(short, long, action = ArgAction::SetTrue)]
    // pub form: bool,

    /// allows you to pass raw request data without extra processing
    #[arg(long)]
    pub raw: Option<String>,

    /// save output to file instead of stdout
    #[arg(short, long)]
    pub output: Option<String>,

    /// do not print the response body to stdout; rather, download it and store it in a file
    #[arg(short, long)]
    pub download: bool,

    // TODO: support --continue (-c)
    // TODO: support --session

    /// basic authentication (user[:password]) or bearer token
    #[arg(short, long)]
    pub auth: Option<String>,

    /// follows Location redirects
    #[arg(short = 'F', long, action = ArgAction::SetTrue)]
    pub follow: bool,

    /// when following redirects, max redirects
    #[arg(long, default_value_t = 30)]
    pub max_redirects: usize,

    /// set to "no" (or "false") to skip checking the host's SSL certificate
    #[arg(long, default_value_t = CliBool::Yes)]
    pub verify: CliBool,

    /// Show headers
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Verb {
    /// performs a CONNECT request
    #[command()]
    Connect {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value and/or querystring==value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a DELETE request
    #[command()]
    Delete {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value and/or querystring==value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a GET request
    #[command()]
    Get {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value and/or querystring==value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a HEAD request
    #[command()]
    Head {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value and/or querystring==value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a OPTION request
    #[command()]
    Option {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value and/or querystring==value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a PATCH request
    #[command()]
    Patch {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value, querystring==value, and/or payload=value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a POST request
    #[command()]
    Post {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value, querystring==value, and/or payload=value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a PUT request
    #[command()]
    Put {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value, querystring==value, and/or payload=value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },

    /// performs a TRACE request
    #[command()]
    Trace {

        /// the URL to connect to
        #[arg()]
        url: Url,

        /// header:value and/or querystring==value; @value means value from file content
        #[arg()]
        params: Vec<Param>,
    },
}

impl Cli {

    pub fn payload(&self) -> Result<Value, Option<eyre::ErrReport>> {
        if let Some(raw) = &self.raw {
            return Ok(Value::String(parse_string(raw.to_string())?));
        }

        let mut payload: Option<Value> = None;
        for param in self.verb.params().iter() {
            if let Param::Payload(param) = param {
                match payload {
                    None => payload.insert(param.clone()).ignore(),

                    Some(Value::Object(ref mut payload)) =>
                        match param {
                            Value::Object(param) => payload.extend(
                                param.iter()
                                    .map(|(k, v)| (k.to_owned(), v.clone()))
                            ).ignore(),
                            _ => return Err(Some(eyre!("invalid payload"))),
                        },

                    Some(_) => return Err(Some(eyre!("invalid payload"))),
                }
            }
        }

        match payload {
            Some(payload) => Ok(payload),
            None => Err(None),
        }
    }
}

impl Verb {

    pub fn url(&self) -> &Url {
        match self {
            Verb::Connect { url, .. } => url,
            Verb::Delete { url, .. } => url,
            Verb::Get { url, .. } => url,
            Verb::Head { url, .. } => url,
            Verb::Option { url, .. } => url,
            Verb::Patch { url, .. } => url,
            Verb::Post { url, .. } => url,
            Verb::Put { url, .. } => url,
            Verb::Trace { url, .. } => url,
        }
    }

    pub fn params(&self) -> &Vec<Param> {
        match self {
            Verb::Connect { params, .. } => params,
            Verb::Delete { params, .. } => params,
            Verb::Get { params, .. } => params,
            Verb::Head { params, .. } => params,
            Verb::Option { params, .. } => params,
            Verb::Patch { params, .. } => params,
            Verb::Post { params, .. } => params,
            Verb::Put { params, .. } => params,
            Verb::Trace { params, .. } => params,
        }
    }
}

impl From<&Verb> for Method {

    fn from(value: &Verb) -> Self {
        match value {
            Verb::Connect { .. } => Method::CONNECT,
            Verb::Delete { .. } => Method::DELETE,
            Verb::Get { .. } => Method::GET,
            Verb::Head { .. } => Method::HEAD,
            Verb::Option { .. } => Method::OPTIONS,
            Verb::Patch { .. } => Method::PATCH,
            Verb::Post { .. } => Method::POST,
            Verb::Put { .. } => Method::PUT,
            Verb::Trace { .. } => Method::TRACE,
        }
    }
}

impl TryFrom<&Cli> for Request {

    type Error = eyre::Error;

    fn try_from(value: &Cli) -> Result<Self, Self::Error> {
        // if value.json && value.form {
        //     return Err(eyre!("--json and --form are mutually exclusive"));
        // }

        let method: Method = (&value.verb).into();
        let mut url = value.verb.url().clone();
        let mut headers: Vec<(String, String)> = vec![];
        let mut close_connection_set = false;
        let mut user_agent_set = false;
        let connection = reqwest::header::CONNECTION.to_string();
        let user_agent = reqwest::header::USER_AGENT.to_string();

        for param in value.verb.params().iter() {
            match param {
                Param::Header(name, value) => {
                    if user_agent == **name {
                        user_agent_set = true;
                    } else if connection == **name {
                        close_connection_set = true;
                    }
                    headers.push((name.to_owned(), value.to_owned()));
                }

                Param::Query(key, value) => url.query_pairs_mut()
                    .append_pair(&key, &value)
                    .ignore(),

                _ => (),
            }
        }
        if !user_agent_set {
            headers.push((
                user_agent.to_owned(),
                format!(
                    "Mozilla/5.0 ({} {}) AppleWebKit/537.36 (KHTML like Gecko) {}/{} Chrome/129.0.0.0 Safari/537.36",
                    consts::OS,
                    consts::ARCH,
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                ),
            ));
        }
        if !close_connection_set {
            headers.push((connection.to_owned(), "close".to_string()));
        }

        let mut request = Request::new(method, url);
        for (name, value) in headers.iter() {
            let _ = request.headers_mut().insert(
                HeaderName::from_str(name)?,
                HeaderValue::from_str(value)?,
            );
        }

        if let Some(auth) = &value.auth {
            if let Some((username, password)) = auth.split_once(':') {
                let auth = format!("{}:{}", username, password);
                let engine = engine::general_purpose::STANDARD;
                let auth = engine.encode(auth.into_bytes());
                let _ = request.headers_mut().insert(
                    reqwest::header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Basic {}", auth))?,
                );
            } else {
                let _ = request.headers_mut().insert(
                    reqwest::header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", auth))?,
                );
            }
        }

        Ok(request)
    }
}

impl From<&Cli> for Policy {

    fn from(value: &Cli) -> Self {
        if value.follow {
            Policy::limited(value.max_redirects)
        } else {
            Policy::none()
        }
    }
}


trait Ignore {
    fn ignore(&self) {}
}

impl<T> Ignore for T {
}
