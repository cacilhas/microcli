use std::sync::LazyLock;

use crossterm::style::{Attribute, Attributes, Color, ContentStyle};


pub static DEFAULT_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default().with(Attribute::Reset);
    ContentStyle {
        foreground_color: None,
        background_color: None,
        underline_color: None,
        attributes,
    }
});

pub static HEADER_NAME_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default().with(Attribute::Bold);
    ContentStyle {
        foreground_color: Some(Color::White),
        background_color: None,
        underline_color: None,
        attributes,
    }
});

pub static HEADER_VALUE_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default();
    ContentStyle {
        foreground_color: Some(Color::Blue),
        background_color: None,
        underline_color: None,
        attributes,
    }
});

pub static METHOD_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default();
    ContentStyle {
        foreground_color: Some(Color::Cyan),
        background_color: None,
        underline_color: None,
        attributes,
    }
});

pub static STATUS_FAILURE_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default().with(Attribute::Bold);
    ContentStyle {
        foreground_color: Some(Color::Red),
        background_color: None,
        underline_color: None,
        attributes,
    }
});

pub static STATUS_OTHER_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default().with(Attribute::Bold);
    ContentStyle {
        foreground_color: Some(Color::Yellow),
        background_color: None,
        underline_color: None,
        attributes,
    }
});

pub static STATUS_SUCCESS_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default().with(Attribute::Bold);
    ContentStyle {
        foreground_color: Some(Color::Green),
        background_color: None,
        underline_color: None,
        attributes,
    }
});

pub static URL_STYLE: LazyLock<ContentStyle> = LazyLock::new(|| {
    let attributes = Attributes::default();
    ContentStyle {
        foreground_color: Some(Color::Yellow),
        background_color: None,
        underline_color: None,
        attributes,
    }
});
