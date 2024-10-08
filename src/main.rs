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
use argh::FromArgs;

mod format;
mod olekit;

#[derive(FromArgs, PartialEq, Debug)]
/// OLEkit.
struct TopLevel {
    #[argh(subcommand)]
    nested: SubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommandEnum {
    Ls(SubCommandLs),
    Cat(SubCommandCat),
}

#[derive(FromArgs, PartialEq, Debug)]
/// List OLE file entries.
#[argh(subcommand, name = "ls")]
pub(crate) struct SubCommandLs {
    /// with -l and --size, print sizes like 1K 234M 2G etc.
    #[argh(switch, short = 'h', long = "human-readable")]
    human: bool,
    /// print the allocated size of each file, in blocks
    #[argh(switch, short = 's')]
    size: bool,
    /// colorize the output
    #[argh(switch, short = 'c')]
    color: bool,
    /// print more details for each entry
    #[argh(switch, short = 'd')]
    details: bool,
    /// print the index number of each file
    #[argh(switch, short = 'i')]
    idirid: bool,
    /// print the full path of each entry
    #[argh(switch, short = 'f')]
    full_path: bool,
    /// an OLE file to analyze
    #[argh(positional)]
    file: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Concatenate entries and print on the standard output.
#[argh(subcommand, name = "cat")]
pub(crate) struct SubCommandCat {
    /// an OLE file to analyze
    #[argh(positional)]
    file: String,
    /// IDs of the entries
    #[argh(positional)]
    ids: Vec<usize>,
}

fn main() {
    let args: TopLevel = argh::from_env();

    match match args.nested {
        SubCommandEnum::Ls(ls) => olekit::ls(&ls),
        SubCommandEnum::Cat(cat) => olekit::cat(&cat),
    } {
        Err(e) => eprintln!("An error has occured: {}", e),
        _ => {}
    };
}
