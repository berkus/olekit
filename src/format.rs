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

use ansi_term::Color;
use humansize::{format_size, DECIMAL};
use jiff::{fmt::rfc2822::DateTimePrinter, Timestamp};
use ole;

const DIF_TIME_WINDOWS: u64 = 116444736000000000u64;

pub(crate) struct Formatter {
    pub(crate) color: bool,
    pub(crate) size: bool,
    pub(crate) human: bool,
    pub(crate) more_details: bool,
    pub(crate) idirid: bool,
    pub(crate) full_path: bool,
}

impl Formatter {
    pub fn print_entries(&self, entries: Vec<&ole::Entry>) {
        let mut total_size = 0u64;

        let mut name = vec![];
        let mut discovered_entries = vec![];
        self.print_entry(&entries, entries[0], &mut name, &mut discovered_entries);

        for entry in entries {
            total_size += entry.len() as u64;
        }
        print!("{}", self.suffix(total_size));
    }

    fn print_entry(
        &self,
        entries: &Vec<&ole::Entry>,
        entry: &ole::Entry,
        dir_stack: &mut Vec<String>,
        discovered_entries: &mut Vec<u32>,
    ) {
        // add entry to the discovered_entries
        discovered_entries.push(entry.id());

        // print the entry
        let mut s = String::new();
        if self.idirid {
            s.push_str(&format!("{:<6} ", entry.id()));
        }
        if self.size {
            s.push_str(&format!("{} ", self.get_string_size(entry.len() as u64)));
        }
        if self.more_details {
            s.push_str(&format!(
                "{:>19} ",
                self.format_date(entry.last_modification_time())
            ));
        }

        let mut name = String::from(entry.name());
        for dir_name in dir_stack.iter().rev() {
            name.insert_str(0, &format!("{}/", dir_name));
        }
        if self.full_path {
            s.push_str(&format!("{}\n", self.format_name(entry, &name)));
        } else {
            s.push_str(&format!("{}\n", self.format_name(entry, entry.name())));
        }
        print!("{}", s);

        // if it's a user stream, we're done. else, go next
        if entry._type() == ole::EntryType::UserStorage
            || entry._type() == ole::EntryType::RootStorage
        {
            if !self.full_path {
                print!("{}:\n", self.format_name(entry, &name));
            }
            dir_stack.push(String::from(entry.name()));
            for entry_id in entry.children_nodes() {
                if !discovered_entries.contains(entry_id) {
                    let new_entry = entries[*entry_id as usize];
                    self.print_entry(entries, new_entry, dir_stack, discovered_entries);
                }
            }
            dir_stack.pop();
        }
    }

    fn suffix(&self, total_size: u64) -> String {
        format!("total {}\n", self.get_string_size(total_size))
    }

    fn get_string_size(&self, size: u64) -> String {
        match self.human {
            false => format!("{:>8}", size),
            true => format!("{:>8}", self.format_human_size(size)),
        }
    }

    fn format_human_size(&self, size: u64) -> String {
        format_size(size, DECIMAL)
    }

    fn format_name(&self, entry: &ole::Entry, name: &str) -> String {
        match self.color {
            false => String::from(name),
            true => match entry._type() {
                ole::EntryType::UserStorage => Color::Blue.paint(name).to_string(),
                ole::EntryType::UserStream => Color::Green.paint(name).to_string(),
                ole::EntryType::RootStorage => Color::Yellow.paint(name).to_string(),
                ole::EntryType::Empty => Color::Red.paint("empty").to_string(),
                _ => String::from(name),
            },
        }
    }

    fn format_date(&self, date: u64) -> String {
        if date == 0 {
            "(no date specified)".into()
        } else {
            Timestamp::from_second(((date - DIF_TIME_WINDOWS) / 10000000) as i64).map_or_else(
                |_| "(date not valid)".into(),
                |t| {
                    DateTimePrinter::new()
                        .timestamp_to_string(&t)
                        .unwrap_or("(failed to format date)".into())
                },
            )
        }
    }
}
