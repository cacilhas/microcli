mod cli_bool;
mod param;
mod util;

use std::{
    collections::HashMap,
    env::consts,
    path::Path,
    str::FromStr,
    sync::LazyLock,
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

static DEFAULT_USER_AGENT: LazyLock<String> = LazyLock::new(||
    format!(
        "Mozilla/5.0 ({} {}) AppleWebKit/537.36 (KHTML like Gecko) {}/{} Chrome/129.0.0.0 Safari/537.36",
        consts::OS,
        consts::ARCH,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
);


pub trait CLParameters {

    fn output(&self) -> Option<String>;
    fn payload(&self) -> Result<Value, Option<eyre::ErrReport>>;
    fn policy(&self) -> Policy;
    fn request(&self) -> eyre::Result<Request>;
    fn url(&self) -> &Url;
    fn verbose(&self) -> bool;
    fn fail(&self) -> bool;
}


#[derive(Debug, Parser)]
#[command(about, author, name = "http", version)]
pub struct Cli {

    #[command(subcommand)]
    verb: Verb,

    #[arg(skip = reqwest::Url::parse("http://localhost/").unwrap())]
    url: Url,
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

    /// save output to file instead of stdout [default: URL path file name]
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

    /// fail on error status code
    #[arg(long, action = ArgAction::SetTrue)]
    fail: bool,

    /// Show headers
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Verb {
    /// performs a CONNECT request
    #[command(aliases = ["Connect", "CONNECT"])]
    Connect(VerbArgs),
    /// performs a DELETE request
    #[command(aliases = ["Delete", "DELETE"])]
    Delete(VerbArgs),
    /// performs a GET request
    #[command(aliases = ["Get", "GET"])]
    Get(VerbArgs),
    /// performs a HEAD request
    #[command(aliases = ["Head", "HEAD"])]
    Head(VerbArgs),
    /// performs a OPTION request
    #[command(aliases = ["Option", "OPTION"])]
    Option(VerbArgs),
    /// performs a PATCH request
    #[command(aliases = ["Patch", "PATCH"])]
    Patch(VerbArgs),
    /// performs a POST request
    #[command(aliases = ["Post", "POST"])]
    Post(VerbArgs),
    /// performs a PUT request
    #[command(aliases = ["Put", "PUT"])]
    Put(VerbArgs),
    /// performs a TRACE request
    #[command(aliases = ["Trace", "TRACE"])]
    Trace(VerbArgs),
}

impl CLParameters for Cli {

    fn output(&self) -> Option<String> {
        let args = self.verb.args();
        match args.output.clone() {
            Some(output) => Some(output),
            None => {
                if args.download {
                    let path = Path::new(self.url().path());
                    path.file_name().map(|path| path.to_string_lossy().to_string())
                } else {
                    None
                }
            }
        }
    }

    fn payload(&self) -> Result<Value, Option<eyre::ErrReport>> {
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

    #[inline]
    #[must_use]
    fn policy(&self) -> Policy {
        self.into()
    }

    #[inline]
    fn request(&self) -> eyre::Result<Request> {
        self.try_into()
    }

    #[inline]
    #[must_use]
    fn url(&self) -> &Url {
        &self.url
    }

    #[inline]
    #[must_use]
    fn verbose(&self) -> bool {
        self.verb.args().verbose
    }

    #[inline]
    #[must_use]
    fn fail(&self) -> bool {
        self.verb.args().fail
    }
}

impl Cli {

    fn headers(&self) -> HashMap<String, String> {

        let args = self.verb.args();
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(reqwest::header::CONNECTION.to_string(), "close".to_string());
        headers.insert(reqwest::header::USER_AGENT.to_string(), DEFAULT_USER_AGENT.to_owned());

        for param in args.params.iter() {
            if let Param::Header(name, value) = param {
                let entry = headers.entry(name.to_lowercase()).or_insert(String::new());
                *entry = value.to_owned();
            }
        }

        headers
    }

    fn build_request(&self) -> eyre::Result<Request> {
        let method: Method = (&self.verb).into();
        let headers = self.headers();
        let mut request = Request::new(method, self.url().clone());
        for (name, value) in headers.iter() {
            let _ = request.headers_mut().insert(
                HeaderName::from_str(name)?,
                HeaderValue::from_str(value)?,
            );
        }
        Ok(self.set_authorization(request))
    }

    fn set_authorization(&self, mut request: Request) -> Request {
        if let Some(auth) = &self.verb.args().auth {
            if let Some((username, password)) = auth.split_once(':') {
                let auth = format!("{}:{}", username, password);
                let engine = engine::general_purpose::STANDARD;
                let auth = engine.encode(auth.into_bytes());
                let _ = request.headers_mut().insert(
                    reqwest::header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Basic {}", auth)).unwrap(),
                );
            } else {
                let _ = request.headers_mut().insert(
                    reqwest::header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", auth)).unwrap(),
                );
            }
        }
        request
    }

    pub fn initialize(&mut self) -> eyre::Result<()> {
        let args = self.verb.args();
        let mut url = args.url.clone();

        // if args.json && args.form {
        //     return Err(eyre!("--json and --form are mutually exclusive"));
        // }

        for param in args.params.iter() {
            if let Param::Query(key, value) = param { url.query_pairs_mut()
            .append_pair(key, value)
            .ignore() }
        }

        self.url = url;

        Ok(())
    }
}

impl TryFrom<&Cli> for Request {

    type Error = eyre::Error;

    fn try_from(value: &Cli) -> Result<Self, Self::Error> {
        Ok(value.build_request()?)
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

impl Verb {

    #[inline]
    #[must_use]
    fn args(&self) -> &VerbArgs {
        match self {
            Verb::Connect(args) => args,
            Verb::Delete(args)  => args,
            Verb::Get(args)     => args,
            Verb::Head(args)    => args,
            Verb::Option(args)  => args,
            Verb::Patch(args)   => args,
            Verb::Post(args)    => args,
            Verb::Put(args)     => args,
            Verb::Trace(args)   => args,
        }
    }
}

impl From<&Verb> for Method {

    #[inline]
    fn from(value: &Verb) -> Self {
        match value {
            Verb::Connect(_) => Method::CONNECT,
            Verb::Delete(_)  => Method::DELETE,
            Verb::Get(_)     => Method::GET,
            Verb::Head(_)    => Method::HEAD,
            Verb::Option(_)  => Method::OPTIONS,
            Verb::Patch(_)   => Method::PATCH,
            Verb::Post(_)    => Method::POST,
            Verb::Put(_)     => Method::PUT,
            Verb::Trace(_)   => Method::TRACE,
        }
    }
}


trait Ignore {

    #[inline]
    fn ignore(&self) {}
}

impl<T> Ignore for T {
}
