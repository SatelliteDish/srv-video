// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// b569177db315321e24dce4bae7bc53d0b5c48ff460c90eae0ffaf38a7cfb147c
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("minus", "\u{E11C}"),
    ("pause", "\u{E12E}"),
    ("play", "\u{E13C}"),
    ("plus", "\u{E13D}"),
    ("x", "\u{E1B2}"),
];

pub fn minus<'a>() -> Text<'a> {
    icon("\u{E11C}")
}

pub fn pause<'a>() -> Text<'a> {
    icon("\u{E12E}")
}

pub fn play<'a>() -> Text<'a> {
    icon("\u{E13C}")
}

pub fn plus<'a>() -> Text<'a> {
    icon("\u{E13D}")
}

pub fn x<'a>() -> Text<'a> {
    icon("\u{E1B2}")
}

/// Render any Lucide icon by its codepoint string.
/// Use this together with [`ALL_ICONS`] to display icons dynamically:
/// ```ignore
/// for (name, cp) in ALL_ICONS {
///     button(render(cp)).on_press(Msg::Pick(name.to_string()))
/// }
/// ```
pub fn render(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("lucide"))
}

fn icon(codepoint: &str) -> Text<'_> {
    render(codepoint)
}
