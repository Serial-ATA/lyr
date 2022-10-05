# Lyr

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Serial-ATA/lyr/CI?style=for-the-badge&logo=github)](https://github.com/Serial-ATA/lyr/actions/workflows/ci.yml)
[![Downloads](https://img.shields.io/crates/d/lyr?style=for-the-badge&logo=rust)](https://crates.io/crates/lyr)
[![Version](https://img.shields.io/crates/v/lyr?style=for-the-badge&logo=rust)](https://crates.io/crates/lyr)

Download and embed lyrics from multiple sources.

## Sources

* [AZLyrics](https://azlyrics.com)
* [Genius](https://genius.com)
* [JahLyrics](https://jah-lyrics.com)
* [Musixmatch](https://www.musixmatch.com)

NOTE: Genius currently has an issue where there will be missing newlines
      between section headers, so the output may look like:

```
[Verse 1]
Foo
Bar Baz
Qux
[Chorus] # Notice this header immediately follows the last line of Verse 1
```

Not sure how to fix this as of now.

## Usage

Fetch and print the lyrics to stdout:
```console
$ lyr --artist="2Pac" --title="Changes"
```

Try to get the artist and title from the tags in the file:

For the list of supported files see [lofty-rs](https://github.com/Serial-ATA/lofty-rs#supported-formats).
```console
# NOTE: This will add the lyrics to the tags of the file
$ lyr --input="some-music-file.mp3"
# Use the `no-embed` flag to prevent this
$ lyr --input="some-music-file.mp3" --no-embed
```

Output the lyrics to a file:
```console
$ lyr --artist="2Pac" --title="Changes" lyrics.txt
```

## Config

This config is stored at `$CONFIG_DIR/lyr/config.toml`.

```toml
# Default flags to append to every command
flags = ''
# The list of fetchers to use when searching for lyrics
# These *ARE* CaSe-sEnSitiVe
fetchers = ['AZLyrics', 'Genius', 'Musixmatch']
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.