use super::Fetcher;

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub struct MusixmatchFetcher;

impl Fetcher for MusixmatchFetcher {
	const NAME: &'static str = "Musixmatch";
	const WORD_SEPARATOR: &'static str = "-";
	const URL_TEMPLATE: &'static str = "https://www.musixmatch.com/lyrics/%artist%/%title%";

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
