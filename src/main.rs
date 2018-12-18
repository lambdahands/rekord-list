extern crate lazy_static;
extern crate prettytable;
extern crate regex;

use prettytable::format;
use prettytable::{cell, row, Table};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, BufReader};
use lazy_static::lazy_static;

fn read_playlist() -> Option<Result<String, std::io::Error>> {
    let args = env::args().filter(|s| !s.starts_with("--")).nth(1);
    args.map(|file_path| {
        let file = File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;
        Ok(String::from_utf8_lossy(&contents).into_owned())
    })
}

fn set_unicode_table_format(table: &mut Table) {
    table.set_format(
        format::FormatBuilder::new()
            .column_separator('│')
            .borders('│')
            .separators(
                &[format::LinePosition::Top],
                format::LineSeparator::new('─', '┬', '┌', '┐'),
            ).separators(
                &[format::LinePosition::Intern],
                format::LineSeparator::new('─', '┼', '├', '┤'),
            ).separators(
                &[format::LinePosition::Bottom],
                format::LineSeparator::new('─', '┴', '└', '┘'),
            ).padding(1, 1)
            .build(),
    );
}

fn add_row(table: &mut Table, caps: &regex::Captures) {
    table.add_row(row![
        &caps["num"],
        &caps["track"],
        &caps["artist"],
        &caps["album"]
    ]);
}

fn process_playlist(playlist: &String) -> Table {
    let no_null = playlist.replace("\x00", "");
    lazy_static! {
        static ref RE: Regex = Regex::new(
            "^(?x)
            (?P<num>\\d+)\\t
            (?P<track>.*?)\\t
            (?P<artist>.*?)\\t
            (?P<_bpm>.*?)\\t
            (?P<_key>.*?)\\t
            (?P<album>.*?)\\t",
        ).unwrap();
    }
    let mut table = Table::new();
    set_unicode_table_format(&mut table);
    table.add_row(row![FYb => "#", "Track", "Artist", "Album"]);
    for line in no_null.lines().skip(1) {
        RE.captures(line).map(|caps| add_row(&mut table, &caps));
    }
    table
}

fn print_help() {
    println!("rekord-list - readable rekordbox KUVO playlist exports\n");
    println!("Usage:");
    println!("  rekord-list playlist-file");
    println!("  rekord-list playlist-file --csv");
}

fn main() {
    match read_playlist() {
        Some(Ok(string)) => {
            let table = process_playlist(&string);
            if let Some(_) = env::args().find(|s| s == "--csv") {
                table.to_csv(stdout()).ok();
            } else {
                table.printstd();
            }
        }
        Some(Err(e)) => println!("{}", e),
        None => print_help(),
    };
}
