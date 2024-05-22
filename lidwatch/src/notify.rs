use notify_rust::{Hint, Notification};

#[derive(Debug)]
pub enum Icon {
    Info,
    Warning,
}

pub fn notify(msg: impl Into<String>, icon: Icon) {
    if let Err(err) = Notification::new()
        .summary("Category:input")
        .body(&msg.into())
        .icon(icon.into())
        .appname("lidwatch")
        .hint(Hint::Category("input".to_owned()))
        .timeout(5000)
        .show()
    {
        eprintln!("{:#?}", err);
    }
}

impl<'a> From<Icon> for &'a str {
    fn from(value: Icon) -> Self {
        match value {
            Icon::Info => "info",
            Icon::Warning => "warn",
        }
    }
}
