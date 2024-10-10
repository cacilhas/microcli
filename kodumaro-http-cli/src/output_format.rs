use eyre::Result;
use json_color::Colorizer;
use serde_json::Value;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};


pub(crate) async fn format_json(body: &Value) -> Result<()> {
    let colorizer = Colorizer::arbitrary();
    match colorizer.colorize_json_str(&serde_json::to_string(&body)?) {
        Ok(body) => println!("{}", body),
        Err(_) => print!("{}", body),
    };
    Ok(())
}

pub(crate) fn format_html(body: &str) -> Result<()> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = match ps.find_syntax_by_extension("html") {
        Some(syntax) => syntax,
        None => {
            eprintln!("failed to find HTML syntax");
            println!("{}", body);
            return Ok(());
        }
    };
    let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (dark)"]);
    for line in LinesWithEndings::from(body) {
        let ranges: Vec<(Style, &str)> =
            h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        print!("{}", escaped);
    }

    Ok(())
}
