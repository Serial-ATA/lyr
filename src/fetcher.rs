use crate::error::{Error, Result};
use crate::utils::create_url;

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FetcherType {
	AZLyrics,
	Genius,
	Musixmatch
}

pub static DEFAULT_FETCHERS: &[FetcherType] = &[FetcherType::AZLyrics, FetcherType::Genius, FetcherType::Musixmatch];

pub(crate) struct Fetcher {
	name: &'static str,
	word_seperator: &'static str,
	apostrophe_needs_sep: bool,
	requires_post_processing: bool,
	url_template: &'static str,
	regex: Regex,
}

pub(crate) static AZLYRICS_FETCHER: Lazy<Fetcher> = Lazy::new(|| {
	Fetcher {
		name: "azlyrics",
		word_seperator: "",
		apostrophe_needs_sep: false,
		requires_post_processing: true,
		url_template: "https://azlyrics.com/lyrics/%artist%/%title%.html",
		regex: RegexBuilder::new(
			r"<!-- Usage of azlyrics\.com content by any third-party lyrics provider is prohibited by our licensing agreement\. Sorry about that\. -->(.*?)</div>"
		)
			.dot_matches_new_line(true)
			.build()
			.unwrap(),
	}
});

pub(crate) static GENIUS_LYRICS_FETCHER: Lazy<Fetcher> = Lazy::new(|| Fetcher {
	name: "Genius",
	word_seperator: "-",
	apostrophe_needs_sep: false,
	requires_post_processing: true,
	url_template: "https://genius.com/%artist%-%title%-lyrics",
	regex: Regex::new(r#"<div.*?class="(?:lyrics|Lyrics__Container).*?>(.*?)</div>"#).unwrap(),
});

pub(crate) static MUSIXMATCH_FETCHER: Lazy<Fetcher> = Lazy::new(|| Fetcher {
	name: "Musixmatch",
	word_seperator: "-",
	apostrophe_needs_sep: true,
	requires_post_processing: false,
	url_template: "https://www.musixmatch.com/lyrics/%artist%/%title%",
	regex: RegexBuilder::new(r#"<span class="lyrics__content__.*?>(.*?)</span>"#)
		.dot_matches_new_line(true)
		.build()
		.unwrap(),
});

pub(crate) async fn fetch(fetcher: &Fetcher, title: &str, artist: &str) -> Result<String> {
	log::info!("Using fetcher: {}", fetcher.name);

	let url = create_url(
		fetcher.url_template,
		fetcher.word_seperator,
		fetcher.apostrophe_needs_sep,
		title,
		artist,
	);

	let response = reqwest::get(url).await?.text().await?;
	if response.is_empty() {
		return Err(Error::NoLyrics);
	}

	let mut result = String::new();
	for match_ in fetcher.regex.captures_iter(response.trim()) {
		result.push_str(&match_[1]);
	}

	if result.is_empty() {
		return Err(Error::NoMatches);
	}

	if !fetcher.requires_post_processing {
		return Ok(result);
	}

	// These newlines interfere with the <br> tags
	result.retain(|c| !(c == '\n' || c == '\r'));

	let stripped = crate::utils::strip_html(&result);
	Ok(stripped)
}
