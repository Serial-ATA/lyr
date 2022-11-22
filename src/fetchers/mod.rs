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

pub(crate) fn fetch(fetcher: &str, title: &str, artist: &str) -> Result<String> {
	match fetcher {
		"azlyrics" => fetch_inner::<AZLyricsFetcher>(title, artist),
		"genius" => fetch_inner::<GeniusFetcher>(title, artist),
		"jahlyrics" => fetch_inner::<JahLyricsFetcher>(title, artist),
		"musixmatch" => fetch_inner::<MusixmatchFetcher>(title, artist),
		_ => unreachable!(),
	}
}

pub(crate) trait Fetcher {
	const NAME: &'static str;
	const WORD_SEPARATOR: &'static str = "";
	const APOSTROPHE_NEEDS_SEP: bool = true;
	const URL_TEMPLATE: &'static str;

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

fn fetch_inner<FETCHER: Fetcher>(title: &str, artist: &str) -> Result<String> {
	log::info!("Using fetcher: {}", FETCHER::NAME);

	let url = crate::utils::create_url(
		FETCHER::URL_TEMPLATE,
		FETCHER::WORD_SEPARATOR,
		FETCHER::APOSTROPHE_NEEDS_SEP,
		title,
		artist,
	);

	let response = reqwest::blocking::get(url)?.text()?;
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
