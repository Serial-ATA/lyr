use crate::error::{Error, Result};
use crate::utils::create_url;

use fancy_regex::Regex;
use once_cell::sync::Lazy;

pub static DEFAULT_FETCHERS: Lazy<Vec<String>> = Lazy::new(|| vec![String::from("genius")]);

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

pub(crate) async fn fetch(fetcher: &Fetcher, title: &str, artist: &str) -> Result<String> {
    log::info!("Using fetcher: {}", fetcher.name);

    let url = create_url(fetcher.url_template, fetcher.word_seperator, title, artist);

    let response = reqwest::get(url).await?.text().await?;
    if response.is_empty() {
        return Err(Error::NoLyrics);
    }

    let mut segments = Vec::new();
    for match_ in fetcher.regex.captures_iter(response.trim()) {
        segments.push(match_.unwrap()[1].to_string());
    }

    let segments_size = segments.len();

    let mut lyrics = String::new();
    for (idx, segment) in segments.iter_mut().enumerate() {
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
