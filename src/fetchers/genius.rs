use super::Fetcher;

use once_cell::sync::Lazy;
use regex::Regex;

pub struct GeniusFetcher;

impl Fetcher for GeniusFetcher {
	const NAME: &'static str = "Genius";
	const WORD_SEPARATOR: &'static str = "-";
	const APOSTROPHE_NEEDS_SEP: bool = false;
	const URL_TEMPLATE: &'static str = "https://genius.com/%artist%-%title%-lyrics";

	fn regex() -> &'static Regex {
		static REGEX: Lazy<Regex> = Lazy::new(|| {
			Regex::new(r#"<div.*?class="(?:lyrics|Lyrics__Container).*?>(.*?)</div>"#).unwrap()
		});

		&REGEX
	}
}
