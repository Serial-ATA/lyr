use std::borrow::Cow;
use std::io::Write;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use env_logger::{Builder, Target, WriteStyle};
use env_logger::fmt::style::{AnsiColor, Color, Style};
use log::{Level, LevelFilter};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

pub fn setup_logger() {
	let mut builder = Builder::new();

	builder
		.format(|buf, record| {
			let color = match record.level() {
				Level::Trace => Color::Ansi(AnsiColor::Cyan),
				Level::Debug => Color::Ansi(AnsiColor::Black),
				Level::Info => Color::Ansi(AnsiColor::Green),
				Level::Warn => Color::Ansi(AnsiColor::Yellow),
				Level::Error => Color::Ansi(AnsiColor::Red),
			};

			let style = Style::new().fg_color(Some(color));
			let level = record.level().to_string().to_ascii_uppercase();
			writeln!(buf, "{style}{level}{style:#}: {}", record.args())
		})
		.filter(None, LevelFilter::Info)
		.parse_default_env()
		.write_style(WriteStyle::Always)
		.target(Target::Stderr)
		.init();
}

fn unescape_html(content: &str) -> String {
	static MATCHER: Lazy<AhoCorasick> = Lazy::new(|| {
		AhoCorasickBuilder::new()
			.match_kind(MatchKind::LeftmostFirst)
			.build([
				"&nbsp;", "&lt;", "&gt;", "&amp;", "&quot;", "&apos;", "&cent;", "&pound;",
				"&yen;", "&euro;", "&copy;", "&reg;", "&ndash;", "&mdash;",
			]).unwrap()
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

	unescape_html(&ret).trim_start().to_string()
}

pub fn create_url(
	template: &str,
	separator: &str,
	apostrophe_needs_sep: bool,
	title: &str,
	artist: &str,
) -> String {
	static FEATURES_REGEX: Lazy<Regex> =
		Lazy::new(|| Regex::new(r" ?\((with|feat)(.*?)\)").unwrap());

	let title = FEATURES_REGEX
		.replacen(title, 0, "")
		.replace('\'', if apostrophe_needs_sep { separator } else { "" });
	let artist = artist.replace('\'', if apostrophe_needs_sep { separator } else { "" });

	template
		.replace("%artist%", &artist.replace(['_', '-', ' '], separator))
		.replace("%title%", &title.replace(['_', '-', ' '], separator))
}
