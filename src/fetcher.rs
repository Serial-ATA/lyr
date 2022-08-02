use crate::error::{Error, Result};
use crate::utils::create_url;

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub static DEFAULT_FETCHERS: &[&str] = &["genius", "azlyrics"];

pub(crate) struct Fetcher {
	name: &'static str,
	word_seperator: &'static str,
	url_template: &'static str,
	regex: Regex,
}

pub(crate) static GENIUS_LYRICS_FETCHER: Lazy<Fetcher> = Lazy::new(|| Fetcher {
	name: "Genius",
	word_seperator: "-",
	url_template: "https://genius.com/%artist%-%title%-lyrics",
	regex: Regex::new(r#"<div.*?class="(?:lyrics|Lyrics__Container).*?>(.*?)</div>"#).unwrap(),
});

pub(crate) static AZLYRICS_FETCHER: Lazy<Fetcher> = Lazy::new(|| {
	Fetcher {
        name: "azlyrics",
        word_seperator: "",
        url_template: "https://azlyrics.com/lyrics/%artist%/%title%.html",
        regex: RegexBuilder::new(
            r"<!-- Usage of azlyrics\.com content by any third-party lyrics provider is prohibited by our licensing agreement\. Sorry about that\. -->(.*?)</div>"
        )
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
    }
});

pub(crate) async fn fetch(fetcher: &Fetcher, title: &str, artist: &str) -> Result<String> {
	log::info!("Using fetcher: {}", fetcher.name);

	let url = create_url(fetcher.url_template, fetcher.word_seperator, title, artist);

	let response = reqwest::get(url).await?.text().await?;
	if response.is_empty() {
		return Err(Error::NoLyrics);
	}

	let mut segments = Vec::new();
	for match_ in fetcher.regex.captures_iter(response.trim()) {
		segments.push(match_[1].to_string());
	}

	if segments.is_empty() {
		return Err(Error::NoMatches);
	}

	let segments_size = segments.len();
	let mut lyrics = String::new();
	for (idx, segment) in segments.iter_mut().enumerate() {
		// These newlines interfere with the <br> tags
		segment.retain(|c| !(c == '\n' || c == '\r'));
		let stripped = crate::utils::strip_html(segment);

		for line in stripped.lines() {
			lyrics.push_str(line);
			lyrics.push('\n');
		}

		if idx != segments_size - 1 {
			lyrics.push('\n')
		}
	}

	Ok(lyrics)
}
