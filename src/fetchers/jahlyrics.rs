use super::Fetcher;

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub struct JahLyricsFetcher;

impl Fetcher for JahLyricsFetcher {
	fn name() -> &'static str {
		"Jah Lyrics"
	}

	fn word_seperator() -> &'static str {
		"-"
	}

	fn url_template() -> &'static str {
		"https://jah-lyrics.com/song/%artist%-%title%"
	}

	fn regex() -> &'static Regex {
		static REGEX: Lazy<Regex> = Lazy::new(|| {
			RegexBuilder::new(r#"<div class="song-header">.*?</div>(.*?)<p class="disclaimer">"#)
				.dot_matches_new_line(true)
				.build()
				.unwrap()
		});

		&REGEX
	}
}
