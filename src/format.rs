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

use ansi_term;
use jiff::{fmt::rfc2822::DateTimePrinter, Timestamp};
use ole;

const DIF_TIME_WINDOWS: u64 = 116444736000000000u64;

pub(crate) trait Formatter {
    fn set_color(&mut self, color: bool);
    fn print_entry(
        &self,
        entries: &std::vec::Vec<&ole::Entry>,
        entry: &ole::Entry,
        dir_stack: &mut std::vec::Vec<std::string::String>,
        discovered_entries: &mut std::vec::Vec<u32>,
    );
    fn color(&self) -> bool;

    // @todo: use humansize crate
    fn format_human_size(&self, size: u64) -> std::string::String {
        let result: std::string::String;
        let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        if size < 1 {
            result = format!("{}{}", size, "B");
        } else {
            let delimiter = 1000f64;
            let exponent = std::cmp::min(
                ((size as f64).ln() / delimiter.ln()).floor() as i32,
                (units.len() - 1) as i32,
            ) as i32;
            let pretty_bytes = format!("{:.2}", size as f64 / delimiter.powi(exponent))
                .parse::<f64>()
                .unwrap()
                * 1_f64;
            let unit = units[exponent as usize];
            result = format!("{}{}", pretty_bytes, unit);
        }

        result
    }

    fn format_name(&self, entry: &ole::Entry, name: &str) -> std::string::String {
        match self.color() {
            false => std::string::String::from(name),
            true => match entry._type() {
                ole::EntryType::UserStorage => ansi_term::Colour::Blue.paint(name).to_string(),
                ole::EntryType::UserStream => ansi_term::Colour::Green.paint(name).to_string(),
                ole::EntryType::RootStorage => ansi_term::Colour::Yellow.paint(name).to_string(),
                ole::EntryType::Empty => ansi_term::Colour::Red.paint("empty").to_string(),
                _ => std::string::String::from(name),
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

    fn print_entries(&self, entries: std::vec::Vec<&ole::Entry>) {
        let mut total_size = 0u64;
        print!("{}", self.print_prefix());

        let mut name = std::vec::Vec::<std::string::String>::new();
        let mut discovered_entries = std::vec::Vec::<u32>::new();
        self.print_entry(&entries, entries[0], &mut name, &mut discovered_entries);

        for entry in entries {
            total_size += entry.len() as u64;
        }
        print!("{}", self.print_suffix(total_size));
    }

    fn print_prefix(&self) -> std::string::String {
        std::string::String::from("")
    }

    fn print_suffix(&self, _total_size: u64) -> std::string::String {
        std::string::String::from("\n")
    }
}

pub(crate) struct DetailsFormatter {
    pub(crate) color: bool,
    pub(crate) size: bool,
    pub(crate) human: bool,
    pub(crate) more_details: bool,
    pub(crate) idirid: bool,
    pub(crate) full_path: bool,
}

impl Formatter for DetailsFormatter {
    fn set_color(&mut self, color: bool) {
        self.color = color;
    }

    fn color(&self) -> bool {
        self.color
    }

    fn print_entry(
        &self,
        entries: &std::vec::Vec<&ole::Entry>,
        entry: &ole::Entry,
        dir_stack: &mut std::vec::Vec<std::string::String>,
        discovered_entries: &mut std::vec::Vec<u32>,
    ) {
        // we add the entry to the discovered_entries
        discovered_entries.push(entry.id());

        // we print the entry
        let mut s = std::string::String::new();
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

        let mut name = std::string::String::from(entry.name());
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
            dir_stack.push(std::string::String::from(entry.name()));
            for entry_id in entry.children_nodes() {
                if !discovered_entries.contains(entry_id) {
                    let new_entry = entries[*entry_id as usize];
                    self.print_entry(entries, new_entry, dir_stack, discovered_entries);
                }
            }
            dir_stack.pop();
        }
    }

    fn print_suffix(&self, total_size: u64) -> std::string::String {
        format!("total {}\n", self.get_string_size(total_size))
    }
}

impl DetailsFormatter {
    fn get_string_size(&self, size: u64) -> std::string::String {
        match self.human {
            false => format!("{:>8}", size),
            true => format!("{:>8}", self.format_human_size(size)),
        }
    }
}
