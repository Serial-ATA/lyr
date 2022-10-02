mod config;
mod error;
mod fetcher;
mod utils;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::fetcher::{AZLYRICS_FETCHER, FetcherType, GENIUS_LYRICS_FETCHER, MUSIXMATCH_FETCHER};

use std::ops::Deref;
use std::path::PathBuf;
use std::{fs, process};

use clap::{Parser, ValueHint};
use lofty::{Accessor, AudioFile, ItemKey};

#[derive(Parser)]
#[clap(name = "lyr")]
#[clap(author = "Serial-ATA")]
#[clap(about = "Fetches and embeds lyrics from multiple sources")]
struct Args {
	#[clap(long, short)]
	title: Option<String>,
	#[clap(long, short)]
	artist: Option<String>,
	#[clap(
        long,
        short,
        required_unless_present_all = &["title", "artist"],
        value_hint = ValueHint::FilePath,
        help = "A file to extract the artist/title from, and optionally embed the lyrics into"
    )]
	input: Option<PathBuf>,
	#[clap(
		long,
		requires = "input",
		help = "If the lyrics should be embedded in the tags of the input file"
	)]
	no_embed: bool,
	#[clap(name = "OUTPUT TEXT FILE", value_hint = ValueHint::FilePath)]
	output: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
	utils::setup_logger();
	let args = Args::parse();

	if let Err(e) = real_main(args).await {
		log::error!("{e}");
		process::exit(-1);
	}
}

async fn real_main(args: Args) -> Result<()> {
	let config = Config::read()?;

	let (title, artist) = {
		if let (Some(title), Some(artist)) = (args.title, args.artist) {
			(title, artist)
		} else {
			let file = lofty::read_from_path(args.input.as_ref().unwrap(), false)?;

			let mut title = None;
			let mut artist = None;
			for tag in file.tags() {
				if title.is_some() && artist.is_some() {
					break;
				}

				title = tag.title().map(str::to_lowercase);
				artist = tag.artist().map(str::to_lowercase);
			}

			if title.is_none() || artist.is_none() {
				return Err(Error::InvalidTags);
			}

			(title.unwrap(), artist.unwrap())
		}
	};

	let mut fetchers = config.fetchers.iter();
	let mut lyrics = None;

	while lyrics.is_none() {
		if let Some(fetcher) = fetchers.next() {
			let fetcher = match fetcher {
				FetcherType::AZLyrics => AZLYRICS_FETCHER.deref(),
				FetcherType::Genius => GENIUS_LYRICS_FETCHER.deref(),
				FetcherType::Musixmatch => MUSIXMATCH_FETCHER.deref(),
			};

			// TODO: verbose flag
			if let Ok(lyrics_) = fetcher::fetch(fetcher, &title, &artist).await {
				lyrics = Some(lyrics_)
			}

			continue;
		}

		break;
	}

	let lyrics = match lyrics {
		Some(lyrics) => lyrics,
		None => return Err(Error::NoLyrics),
	};

	if let Some(ref output) = args.output {
		fs::write(output, &lyrics)?;
	}

	if let Some(ref input) = args.input {
		if !args.no_embed {
			let mut file = lofty::read_from_path(input, false)?;
			let contains_tags = file.contains_tag();

			let tag = match file.primary_tag_mut() {
				Some(t) => t,
				_ if contains_tags => file.first_tag_mut().unwrap(),
				_ => {
					log::warn!("The input file doesn't have any eligible tags to write the lyrics");
					process::exit(0);
				},
			};

			tag.insert_text(ItemKey::Lyrics, lyrics);
			file.save_to_path(input)?;
		}
	}

	Ok(())
}
