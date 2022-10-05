use super::Fetcher;

use once_cell::sync::Lazy;
use regex::Regex;

pub struct GeniusFetcher;

impl Fetcher for GeniusFetcher {
	fn name() -> &'static str {
		"Genius"
	}

	fn word_seperator() -> &'static str {
		"-"
	}

	fn apostrophe_needs_sep() -> bool {
		false
	}

	fn url_template() -> &'static str {
		"https://genius.com/%artist%-%title%-lyrics"
	}

	fn regex() -> &'static Regex {
		static REGEX: Lazy<Regex> = Lazy::new(|| {
			Regex::new(r#"<div.*?class="(?:lyrics|Lyrics__Container).*?>(.*?)</div>"#).unwrap()
		});

		&REGEX
	}
}
