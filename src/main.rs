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

extern crate ole;
extern crate ansi_term;
extern crate clap;

mod olekit;
mod format;

fn main() {
  let matches = clap::App::new("OLEkit")
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(
      clap::SubCommand::with_name("ls")
        .about("List OLE file entries")
        .arg(clap::Arg::with_name("FILE")
          .help("OLE file to analyze")
          .required(true)
          .index(1)
        )
        .arg(clap::Arg::with_name("human")
          .short("h")
          .long("human-readable")
          .required(false)
          .takes_value(false)
          .help("with -l and -s, print sizes like 1K 234M 2G etc.")
        )
        .arg(clap::Arg::with_name("size")
          .short("s")
          .long("size")
          .required(false)
          .takes_value(false)
          .help("print the allocated size of each file, in blocks")
        )
        .arg(clap::Arg::with_name("color")
          .long("color")
          .required(false)
          .takes_value(false)
          .help("colorize the output")
        )
        .arg(clap::Arg::with_name("details")
          .short("d")
          .long("details")
          .required(false)
          .takes_value(false)
          .help("print more details for each entry")
        )
        .arg(clap::Arg::with_name("idirid")
          .short("i")
          .long("idirid")
          .required(false)
          .takes_value(false)
          .help("print the index number of each file")
        )
        .arg(clap::Arg::with_name("full-path")
          .short("f")
          .long("full-path")
          .required(false)
          .takes_value(false)
          .help("print the full path of each entry")
        )
    )
    .subcommand(
      clap::SubCommand::with_name("cat")
        .about("Concatenate entries and print on the standard output")
        .arg(clap::Arg::with_name("FILE")
          .help("OLE file to analyze")
          .required(true)
          .index(1)
        )
        .arg(clap::Arg::with_name("ID")
          .help("IDs of the entries")
          .required(true)
          .index(2)
          .min_values(1)
        )
    )
    .get_matches();
  match olekit::olekit(&matches) {
    Err(e) => eprintln!("An error has occured: {}", e),
    _ => {}
  };
}
