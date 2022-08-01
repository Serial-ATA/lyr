use std::borrow::Cow;
use std::io::Write;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use env_logger::fmt::Color;
use env_logger::{Builder, WriteStyle};
use fancy_regex::{Captures, Regex};
use log::{Level, LevelFilter};
use once_cell::sync::Lazy;

pub fn setup_logger() {
    let mut builder = Builder::new();

    builder
        .format(|buf, record| {
            let mut style = buf.style();

            let level = match record.level() {
                Level::Trace => style.set_color(Color::Cyan).value("TRACE"),
                Level::Debug => style.set_color(Color::Black).value("DEBUG"),
                Level::Info => style.set_color(Color::Green).value("INFO "),
                Level::Warn => style.set_color(Color::Yellow).value("WARN "),
                Level::Error => style.set_color(Color::Red).value("ERROR"),
            };

            writeln!(buf, "{}: {}", level, record.args())
        })
        .filter(None, LevelFilter::Info)
        .write_style(WriteStyle::Always)
        .init();
}

fn unescape_html(content: &str) -> String {
    static MATCHER: Lazy<AhoCorasick> = Lazy::new(|| {
        AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .build(&[
                "&nbsp;", "&lt;", "&gt;", "&amp;", "&quot;", "&apos;", "&cent;", "&pound;",
                "&yen;", "&euro;", "&copy;", "&reg;", "&ndash;", "&mdash;",
            ])
    });

    MATCHER.replace_all(
        content,
        &[
            " ", "<", ">", "&", "\"", "'", "¢", "£", "¥", "€", "©", "®", "–", "—",
        ],
    )
}

#[rustfmt::skip]
fn unescape_utf8(content: &str) -> Cow<str> {
    static UNESCAPE_UTF8_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"&#(\w{0,5});").unwrap());

    UNESCAPE_UTF8_REGEX.replace_all(content, |c: &Captures| {
        let cont = &c[1];
        let radix = if &cont[..1] == "x" { 16 } else { 10 };
        let n = u32::from_str_radix(&cont[1..], radix).unwrap();

        format!("{}", char::from_u32(n).unwrap())
    })
}

pub fn strip_html(content: &str) -> String {
    static BR_ELEM_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"</?(?:br|p)(/?|(.*?))>").unwrap());
    static HTML_ELEM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]+>").unwrap());

    let content = unescape_utf8(content);

    let stripped_br = BR_ELEM_REGEX.replace_all(&content, "\n");
    let ret = HTML_ELEM_REGEX.replace_all(&stripped_br, "");

    unescape_html(&ret)
}

pub fn create_url(template: &str, separator: &str, title: &str, artist: &str) -> String {
    static FEATURES_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r" ?\((with|feat)(.*?)\)").unwrap());

    let title = FEATURES_REGEX.replacen(title, 0, "");

    template
        .replace("%artist%", &artist.replace(&['_', '-', ' '], separator))
        .replace("%title%", &title.replace(&['_', '-', ' '], separator))
}
