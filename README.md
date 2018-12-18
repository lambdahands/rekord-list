# rekord-list

Formatting readable Rekordbox KUVO playlists

## Motivation

Rekordbox doesn't allow you to export nicely formatted playlists. This tool can
take a text file created from the `Export a playlist to a file for KUVO (*.txt)`
option in Rekordbox and format it as a pretty unicode table or as a CSV.

## Installation

If you don't have the Rust toolchain, install [rustup](https://rustup.rs/).

In the project top level run:

```
$ cargo build --release
```

You may also move the binary somewhere in your path:

```
$ mv target/release/rekord-list /usr/local/bin
```

## Usage

```
rekord-list - readable rekordbox KUVO playlist exports

Usage:
  rekord-list playlist-file
  rekord-list playlist-file --csv
```
