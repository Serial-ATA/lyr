mod azlyrics;
mod genius;
mod jahlyrics;
mod musixmatch;

use crate::{Error, Result};
use azlyrics::AZLyricsFetcher;
use genius::GeniusFetcher;
use jahlyrics::JahLyricsFetcher;
use musixmatch::MusixmatchFetcher;

use regex::Regex;

pub(crate) const FETCHERS: &[&str] = &["azlyrics", "genius", "jahlyrics", "musixmatch"];

pub(crate) async fn fetch(fetcher: &str, title: &str, artist: &str) -> Result<String> {
	match fetcher {
		"azlyrics" => fetch_inner::<AZLyricsFetcher>(title, artist).await,
		"genius" => fetch_inner::<GeniusFetcher>(title, artist).await,
		"jahlyrics" => fetch_inner::<JahLyricsFetcher>(title, artist).await,
		"musixmatch" => fetch_inner::<MusixmatchFetcher>(title, artist).await,
		_ => unreachable!(),
	}
}

pub(crate) trait Fetcher {
	fn name() -> &'static str;
	fn word_seperator() -> &'static str {
		""
	}
	fn apostrophe_needs_sep() -> bool {
		true
	}
	fn url_template() -> &'static str;
	fn regex() -> &'static Regex;
	fn post_process(input: &mut String) {
		default_post_process(input);
	}
}

fn default_post_process(input: &mut String) {
	// These newlines interfere with the <br> tags
	input.retain(|c| !(c == '\n' || c == '\r'));

	*input = crate::utils::strip_html(input.as_str());
}

async fn fetch_inner<FETCHER: Fetcher>(title: &str, artist: &str) -> Result<String> {
	log::info!("Using fetcher: {}", FETCHER::name());

	let url = crate::utils::create_url(
		FETCHER::url_template(),
		FETCHER::word_seperator(),
		FETCHER::apostrophe_needs_sep(),
		title,
		artist,
	);

	let response = reqwest::get(url).await?.text().await?;
	if response.is_empty() {
		return Err(Error::NoLyrics);
	}

	let mut result = String::new();
	for match_ in FETCHER::regex().captures_iter(response.trim()) {
		result.push_str(&match_[1]);
	}

	if result.is_empty() {
		return Err(Error::NoMatches);
	}

	FETCHER::post_process(&mut result);
	Ok(result)
}
