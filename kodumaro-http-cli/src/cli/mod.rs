mod cli_bool;
mod param;
mod util;

use std::{
    env::consts,
    str::FromStr,
};

use base64::{engine, Engine};
use clap::{ArgAction, Args, Parser, Subcommand};
use cli_bool::CliBool;
use eyre::eyre;
pub use param::Param;
use reqwest::{
    header::{HeaderName, HeaderValue},
    redirect::Policy,
    Method,
    Request,
    Url,
};
use serde_json::Value;
use util::parse_string;

#[derive(Debug, Parser)]
#[command(about, author, name = "http", version)]
pub struct Cli {

    #[command(subcommand)]
    verb: Verb,
}

#[derive(Args, Debug)]
struct VerbArgs {

    /// the URL to connect to
    #[arg()]
    url: Url,

    /// header:value, querystring==value, and/or payload=value; @value means value from file content
    #[arg()]
    params: Vec<Param>,

    // /// data items from the command line are serialized as a JSON object
    // #[arg(short, long, action = ArgAction::SetTrue, default_value_t = true)]
    // pub json: bool,

    // /// data items from the command line are serialized as form fields
    // #[arg(short, long, action = ArgAction::SetTrue)]
    // pub form: bool,

    /// allows you to pass raw request data without extra processing
    #[arg(long)]
    raw: Option<String>,

    /// save output to file instead of stdout
    #[arg(short, long)]
    output: Option<String>,

    /// do not print the response body to stdout; rather, download it and store it in a file
    #[arg(short, long)]
    download: bool,

    // TODO: support --continue (-c)
    // TODO: support --session

    /// basic authentication (user[:password]) or bearer token
    #[arg(short, long)]
    auth: Option<String>,

    /// follows Location redirects
    #[arg(short = 'F', long, action = ArgAction::SetTrue)]
    follow: bool,

    /// when following redirects, max redirects
    #[arg(long, default_value_t = 30)]
    max_redirects: usize,

    /// set to "no" to skip checking the host's SSL certificate
    #[arg(long, default_value_t = CliBool::Yes)]
    verify: CliBool,

    /// Show headers
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Verb {
    #[command(about = "performs a CONNECT request")]
    Connect(VerbArgs),
    #[command(about = "performs a DELETE request")]
    Delete(VerbArgs),
    #[command(about = "performs a GET request")]
    Get(VerbArgs),
    #[command(about = "performs a HEAD request")]
    Head(VerbArgs),
    #[command(about = "performs a OPTION request")]
    Option(VerbArgs),
    #[command(about = "performs a PATCH request")]
    Patch(VerbArgs),
    #[command(about = "performs a POST request")]
    Post(VerbArgs),
    #[command(about = "performs a PUT request")]
    Put(VerbArgs),
    #[command(about = "performs a TRACE request")]
    Trace(VerbArgs),
}

impl Cli {

    pub fn download(&self) -> bool {
        self.verb.args().download
    }

    pub fn output(&self) -> Option<String> {
        self.verb.args().output.clone()
    }

    pub fn payload(&self) -> Result<Value, Option<eyre::ErrReport>> {
        let args = self.verb.args();

        if let Some(raw) = &args.raw {
            return Ok(Value::String(parse_string(raw.to_string())?));
        }

        let mut payload: Option<Value> = None;
        for param in args.params.iter() {
            if let Param::Payload(param) = param {
                match payload {
                    None => payload.insert(param.clone()).ignore(),

                    Some(Value::Object(ref mut payload)) =>
                        match param {
                            Value::Object(param) =>
                                {
                                    payload.extend(
                                        param.iter()
                                            .map(|(k, v)| (k.to_owned(), v.clone()))
                                    );
                                },
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

    pub fn url(&self) -> &Url {
        &self.verb.args().url
    }

    pub fn verbose(&self) -> bool {
        self.verb.args().verbose
    }
}

impl Verb {

    fn args(&self) -> &VerbArgs {
        match self {
            Verb::Connect(args) => args,
            Verb::Delete (args) => args,
            Verb::Get (args) => args,
            Verb::Head (args) => args,
            Verb::Option (args) => args,
            Verb::Patch (args) => args,
            Verb::Post (args) => args,
            Verb::Put (args) => args,
            Verb::Trace (args) => args,
        }
    }
}

impl From<&Verb> for Method {

    fn from(value: &Verb) -> Self {
        match value {
            Verb::Connect { .. } => Method::CONNECT,
            Verb::Delete { .. }  => Method::DELETE,
            Verb::Get { .. }     => Method::GET,
            Verb::Head { .. }    => Method::HEAD,
            Verb::Option { .. }  => Method::OPTIONS,
            Verb::Patch { .. }   => Method::PATCH,
            Verb::Post { .. }    => Method::POST,
            Verb::Put { .. }     => Method::PUT,
            Verb::Trace { .. }   => Method::TRACE,
        }
    }
}

impl TryFrom<&Cli> for Request {

    type Error = eyre::Error;

    fn try_from(value: &Cli) -> Result<Self, Self::Error> {
        let args = value.verb.args();

        // if value.json && value.form {
        //     return Err(eyre!("--json and --form are mutually exclusive"));
        // }

        let method: Method = (&value.verb).into();
        let mut url = args.url.clone();
        let mut headers: Vec<(String, String)> = vec![];
        let mut close_connection_set = false;
        let mut user_agent_set = false;
        let connection = reqwest::header::CONNECTION.to_string();
        let user_agent = reqwest::header::USER_AGENT.to_string();

        for param in args.params.iter() {
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
                    .append_pair(key, value)
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

        if let Some(auth) = &args.auth {
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
        let args = value.verb.args();
        if args.follow {
            Policy::limited(args.max_redirects)
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
