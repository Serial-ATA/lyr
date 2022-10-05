use super::Fetcher;

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub struct MusixmatchFetcher;

impl Fetcher for MusixmatchFetcher {
	fn name() -> &'static str {
		"Musixmatch"
	}

	fn word_seperator() -> &'static str {
		"-"
	}

	fn url_template() -> &'static str {
		"https://www.musixmatch.com/lyrics/%artist%/%title%"
	}

	fn regex() -> &'static Regex {
		static REGEX: Lazy<Regex> = Lazy::new(|| {
			RegexBuilder::new(r#"<span class="lyrics__content__.*?>(.*?)</span>"#)
				.dot_matches_new_line(true)
				.build()
				.unwrap()
		});

		&REGEX
	}

	fn post_process(_input: &mut String) {}
}
