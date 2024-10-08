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

use crate::{SubCommandCat, SubCommandLs};
use ole;

pub(crate) fn ls(ls: &SubCommandLs) -> Result<(), Box<dyn std::error::Error>> {
    let parser = ole::Reader::from_path(&ls.file)?;

    let formatter = crate::format::Formatter {
        human: ls.human,
        size: ls.size,
        color: ls.color,
        more_details: ls.details,
        idirid: ls.idirid,
        full_path: ls.full_path,
    };

    formatter.print_entries(parser.iterate().collect());
    Ok(())
}

pub(crate) fn cat(cat: &SubCommandCat) -> Result<(), Box<dyn std::error::Error>> {
    let parser = ole::Reader::from_path(&cat.file)?;
    let entries: Vec<&ole::Entry> = parser.iterate().collect();

    for &id in &cat.ids {
        if id < entries.len() {
            let entry = entries[id];
            let mut buf = Vec::<u8>::new();
            if let Ok(mut slice) = parser.get_entry_slice(entry) {
                use std::io::{Read, Write};
                match slice.read_to_end(&mut buf) {
                    Ok(_n) => {
                        std::io::stdout().write_all(&buf)?;
                        std::io::stdout().flush()?;
                    }
                    Err(e) => eprintln!("Can't print entry {}: {}", id, e),
                }
            } else {
                eprintln!("Can't get entry {}.", id);
            }
        } else {
            eprintln!("Entry {} doesn't exist", id);
        }
    }
    Ok(())
}
