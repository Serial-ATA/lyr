#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::single_match_else,
	clippy::module_name_repetitions,
	clippy::uninlined_format_args
)]

mod config;
mod error;
mod fetchers;
mod utils;

use crate::config::Config;
use crate::error::{Error, Result};

use std::path::PathBuf;
use std::{fs, process};

use clap::{Parser, ValueHint};
use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::{Accessor, ItemKey};

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

fn main() {
	utils::setup_logger();
	let args = Args::parse();

	if let Err(e) = real_main(args) {
		log::error!("{e}");
		process::exit(-1);
	}
}

fn real_main(args: Args) -> Result<()> {
	let config = Config::read()?;

	let (title, artist);

	if let (Some(arg_title), Some(arg_artist)) = (args.title, args.artist) {
		title = arg_title.to_lowercase();
		artist = arg_artist.to_lowercase();
	} else {
		let file = Probe::open(args.input.as_ref().unwrap())?
			.options(ParseOptions::new().read_properties(false))
			.read()?;

		let mut tag_title = None;
		let mut tag_artist = None;
		for tag in file.tags() {
			if tag_title.is_some() && tag_artist.is_some() {
				break;
			}

			tag_title = tag.title().map(|title| title.to_lowercase());
			tag_artist = tag.artist().map(|artist| artist.to_lowercase());
		}

		let (Some(tag_title), Some(tag_artist)) = (tag_title, tag_artist) else {
			return Err(Error::InvalidTags);
		};

		title = tag_title;
		artist = tag_artist;
	}

	let mut fetchers = config.fetchers.iter();
	let lyrics;

	loop {
		if let Some(fetcher) = fetchers.next() {
			// TODO: verbose flag
			if let Ok(lyrics_) = fetchers::fetch(fetcher, &title, &artist) {
				lyrics = lyrics_;
				break;
			}

			continue;
		}

		// Ran out of fetchers to check
		return Err(Error::NoLyrics);
	}

	if args.output.is_none() && (args.input.is_none() || args.no_embed) {
		println!("{}", lyrics);
		return Ok(());
	}

	if let Some(ref output) = args.output {
		fs::write(output, &lyrics)?;
	}

	if let Some(ref input) = args.input {
		if !args.no_embed {
			let mut file = Probe::open(input)?
				.options(ParseOptions::new().read_properties(false))
				.read()?;
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
			file.save_to_path(input, WriteOptions::default())?;
		}
	}

	Ok(())
}
