use super::Fetcher;

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub struct AZLyricsFetcher;

impl Fetcher for AZLyricsFetcher {
	fn name() -> &'static str {
		"AZLyrics"
	}

	fn apostrophe_needs_sep() -> bool {
		false
	}

	fn url_template() -> &'static str {
		"https://azlyrics.com/lyrics/%artist%/%title%.html"
	}

	fn regex() -> &'static Regex {
		static REGEX: Lazy<Regex> = Lazy::new(|| {
			RegexBuilder::new(
            r"<!-- Usage of azlyrics\.com content by any third-party lyrics provider is prohibited by our licensing agreement\. Sorry about that\. -->(.*?)</div>"
        )
            .dot_matches_new_line(true)
            .build()
            .unwrap()
		});

		&REGEX
	}
}
