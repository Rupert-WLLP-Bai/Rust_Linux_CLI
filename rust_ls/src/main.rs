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
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let entries = list_directory(path);

    for entry in entries {
        println!("{}", entry);
    }
}

fn list_directory(path: &str) -> Vec<String> {
    let mut entries = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|entry| {
            let dir_entry = entry.unwrap();
            let metadata = dir_entry.metadata().unwrap();
            let file_name = dir_entry.file_name().into_string().unwrap();
            #[cfg(windows)]
            let permissions = metadata.file_attributes().to_string(); 
            #[cfg(not(windows))]
            let permissions = metadata.permissions().mode().to_string(); 

            let modified_time = metadata.modified().unwrap();
            let formatted_time = Local.timestamp_opt(modified_time.elapsed().unwrap().as_secs() as i64, 0).unwrap().format("%b %d %H:%M").to_string();

            format!("{:>10} {} {} {} {}", 
                    permissions, 
                    metadata.len(),
                    modified_time.elapsed().unwrap().as_secs(),
                    formatted_time,
                    file_name)
        })
        .collect::<Vec<String>>();

    entries.sort();

    entries
}
