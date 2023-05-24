use std::fs;
use clap::{App, Arg};
#[cfg(windows)]
use std::os::windows::fs::MetadataExt; // for windows only
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt; // for non-windows only
use chrono::Local;
use chrono::prelude::*;

fn main() {
    let matches = App::new("ls")
        .arg(Arg::new("path").default_value(".").index(1))
        .arg(
            Arg::new("long")
                .short('l')
                .help("Use a long listing format"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .help("Do not ignore entries starting with ."),
        )
        .arg(
            Arg::new("human-readable")
                .short('h')
                .help("Human-readable file sizes"),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let show_hidden = matches.is_present("all");
    let show_long_format = matches.is_present("long");
    let show_human_readable = matches.is_present("human-readable");

    let mut entries = list_directory(path, show_hidden);

    if show_long_format {
        for entry in &mut entries {
            let metadata = fs::metadata(&entry.path).unwrap();

            #[cfg(windows)]
            let permissions = metadata.file_attributes().to_string();
            #[cfg(not(windows))]
            let permissions = format!("{:#o}", metadata.permissions().mode() & 0o7777);

            let modified_time = metadata.modified().unwrap();
            let formatted_time = Local.timestamp_opt(modified_time.elapsed().unwrap().as_secs() as i64, 0)
                .unwrap()
                .format("%b %e %H:%M")
                .to_string();

            let size = metadata.len();
            let size_string = if show_human_readable {
                human_readable_size(size)
            } else {
                size.to_string()
            };

            entry.long_format = format!("{:>10} {} {} {}", permissions, formatted_time, size_string, entry.file_name);
        }
    }

    entries.sort();

    for entry in entries {
        if show_long_format {
            println!("{}", entry.long_format);
        } else {
            println!("{}", entry.file_name);
        }
    }
}

#[derive(Ord,PartialEq, Eq,PartialOrd)]
struct DirectoryEntry {
    file_name: String,
    long_format: String,
    path: std::path::PathBuf,
}

fn list_directory(path: &str, show_hidden: bool) -> Vec<DirectoryEntry> {
    let mut entries = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|entry| {
            let dir_entry = entry.unwrap();
            let file_name = dir_entry.file_name().into_string().unwrap();
            let path = dir_entry.path();

            if !show_hidden && file_name.starts_with('.') {
                return None;
            }

            Some(DirectoryEntry {
                file_name,
                long_format: "".to_owned(),
                path,
            })
        })
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    entries.sort_by_key(|entry| entry.file_name.clone());

    entries
}

fn human_readable_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, units[unit_index])
}
