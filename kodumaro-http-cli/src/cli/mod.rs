mod cli_bool;
mod param;
mod util;

use std::str::FromStr;

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

    /// data items from the command line are serialized as a JSON object
    #[arg(short, long, action = ArgAction::SetTrue, default_value_t = true)]
    pub json: bool,

    /// data items from the command line are serialized as form fields
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub form: bool,

    /// similar to --form, but always sends a multipart/form-data request (i.e., even without files)
    #[arg(long, action = ArgAction::SetTrue)]
    pub multipart: bool,

    /// specifies a custom boundary string for multipart/form-data requests
    #[arg(long)]
    pub boundary: Option<String>,

    /// allows you to pass raw request data without extra processing
    #[arg(long)]
    pub raw: Option<String>,

    /// save output to file instead of stdout
    #[arg(short, long)]
    pub output: Option<String>,

    /// do not print the response body to stdout; Rather, download it and store it in a file
    #[arg(short, long)]
    pub download: bool,

    // TODO: support --continue (-c)
    // TODO: support --session

    /// basic authentication (user[:password])
    #[arg(short, long)]
    pub auth: Option<String>,

    /// follows Location redirects
    #[arg(short = 'F', long, action = ArgAction::SetTrue)]
    pub follow: bool,

    /// by default, requests have a limit of 30 redirects
    #[arg(long, default_value_t = 30)]
    pub max_redirects: usize,

    /// set to "no" (or "false") to skip checking the host's SSL certificate; defaults to "yes" ("true")
    #[arg(long, default_value_t = CliBool::Yes)]
    pub verify: CliBool,

    /// Show headers
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    #[command(subcommand)]
    pub verb: Verb,
}

#[derive(Debug, Subcommand)]
pub enum Verb {
    /// performs a CONNECT request
    #[command()]
    Connect {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a DELETE request
    #[command()]
    Delete {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a GET request
    #[command()]
    Get {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a HEAD request
    #[command()]
    Head {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a OPTION request
    #[command()]
    Option {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a PATCH request
    #[command()]
    Patch {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a POST request
    #[command()]
    Post {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a PUT request
    #[command()]
    Put {
        #[arg()]
        url: Url,

        #[arg()]
        params: Vec<Param>,
    },

    /// performs a TRACE request
    #[command()]
    Trace {
        #[arg()]
        url: Url,

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
        if (value.json && value.form)
        || (value.json && value.multipart)
        || (value.form && value.multipart) {
            return Err(eyre!("--json, --form, and --multipart are mutually exclusive"));
        }

        let method: Method = (&value.verb).into();
        let mut url = value.verb.url().clone();
        let mut headers: Vec<(String, String)> = vec![];

        for param in value.verb.params().iter() {
            match param {
                Param::Header(name,  value) => headers
                    .push((name.to_owned(), value.to_owned()))
                    .ignore(),

                Param::Query(key, value) => url.query_pairs_mut()
                    .append_pair(&key, &value)
                    .ignore(),

                _ => (),
            }
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