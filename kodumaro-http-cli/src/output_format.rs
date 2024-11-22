use std::io::Write;

use eyre::Result;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};


pub(crate) fn format_by_ext(body: &str, filename: &str, writer: &mut impl Write) -> Result<()> {
    let ext = filename.split('.').last().unwrap_or("txt");
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    match ps.find_syntax_by_extension(ext) {
        Some(syntax) => {
            let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (dark)"]);
            for line in LinesWithEndings::from(body) {
                let ranges: Vec<(Style, &str)> =
                    h.highlight_line(line, &ps).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                write!(writer, "{}", escaped)?;
            }
        }

        None => {
            write!(writer, "{}", body)?;
            write!(writer, "\n")?;
        }
    }

    Ok(())
}

pub(crate) fn add_ext(filename: &str, content_type: &str) -> String {
    if content_type.contains("markdown") {
        format!("{}.md", filename)
    } else if content_type.contains("html") {
        format!("{}.html", filename)
    } else if content_type.contains("json") {
        format!("{}.json", filename)
    } else if content_type.contains("toml") {
        format!("{}.toml", filename)
    } else if content_type.contains("xml") {
        format!("{}.xml", filename)
    } else if content_type.contains("yaml") {
        format!("{}.yaml", filename)
    } else {
        filename.to_string()
    }
}
