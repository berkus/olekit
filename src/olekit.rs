//             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2018 Thomas Bailleux <thomas@bailleux.me>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
//
// Author: zadig <thomas chr(0x40) bailleux.me>

use std;
use clap;
use ole;


pub(crate) fn olekit(all_matches: &clap::ArgMatches)
      -> Result<(), std::boxed::Box<std::error::Error>> {

  // ls
  if let Some(matches) = all_matches.subcommand_matches("ls") {
    let file = matches.value_of("FILE").unwrap();
    let parser = ole::Reader::from_path(file)?;
    let human = matches.is_present("human");
    let size = matches.is_present("size");
    let color = matches.is_present("color");
    let more_details = matches.is_present("details");
    let idirid = matches.is_present("idirid");
    let full_path = matches.is_present("full-path");
    let formatter: Box<super::format::Formatter>;

    formatter = Box::new(super::format::DetailsFormatter {
        human: human,
        size: size,
        color: color,
        more_details: more_details,
        idirid: idirid,
        full_path: full_path
      });

    formatter.print_entries(parser.iterate().collect());
  }
  // cat
  else if let Some(matches) = all_matches.subcommand_matches("cat") {
    let file = matches.value_of("FILE").unwrap();
    let parser = ole::Reader::from_path(file)?;
    let entries: std::vec::Vec<&ole::Entry> = parser.iterate().collect();
    let files: std::vec::Vec<&str>
        = matches.values_of("ID").unwrap().collect();
    for sid in files {
      if let Ok(id) = sid.parse::<usize>() {
        if id < entries.len() {
          let entry = entries[id];
          let mut buf = std::vec::Vec::<u8>::new();
          if let Ok(mut slice) = parser.get_entry_slice(entry) {
            use std::io::{Read, Write};
            match slice.read_to_end(&mut buf) {
              Ok(_n) => {
                std::io::stdout().write_all(&buf)?;
                std::io::stdout().flush()?;
              },
              Err(e) => eprintln!("Can't print entry {}: {}", id, e)
            }
          } else {
            eprintln!("Can't print entry {}.", id);
          }
        } else {
          eprintln!("Entry {} doesn't exist", id);
        }
      } else {
        eprintln!("Invalid entry id: {}", sid);
      }
    }
  }
  Ok(())
}
