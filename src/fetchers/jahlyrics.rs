use super::Fetcher;

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub struct JahLyricsFetcher;

impl Fetcher for JahLyricsFetcher {
	const NAME: &'static str = "Jah Lyrics";
	const WORD_SEPARATOR: &'static str = "-";
	const APOSTROPHE_NEEDS_SEP: bool = false;
	const URL_TEMPLATE: &'static str = "https://jah-lyrics.com/song/%artist%-%title%";

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
