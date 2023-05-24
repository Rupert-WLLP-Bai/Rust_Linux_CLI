//! # 使用Rust编写的ls命令的简单实现
//!
//! ## 用法: ls [path] [options]
//! - -l: 使用长格式列表
//! - -a: 不忽略以'.'开头的条目
//! - -h: 以人类可读的文件大小显示
//!
//! 默认情况下，将显示当前目录的内容
//!
//! ## 例子
//! ```text
//! ls
//! ls -l, ls -a, ls -h
//! ls -la, ls -lh, ls -ah
//! ls -lah
//! ```
//!
//! ## 改进和进一步实现的功能
//!
//! - 支持排序功能，例如按名称、大小、修改时间等排序
//! - 支持递归显示子目录中的文件和目录
//! - 支持过滤文件类型，例如只显示目录或只显示文件
//! - 支持颜色输出，以更好地区分文件和目录
//! - 支持更多的文件属性，例如所有者、权限等
//!
//! 作者：BJH
//! 日期：2023.05.24
//! 版本：0.1.0

use std::fs;
use clap::{App, Arg};
#[cfg(windows)]
use std::os::windows::fs::MetadataExt; // 仅适用于Windows
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt; // 仅适用于非Windows
use chrono::Local;
use chrono::prelude::*;


/// 主函数
pub fn main() {
    // 创建命令行解析器
    let matches = App::new("ls")
        .arg(Arg::new("path").default_value(".").index(1))
        .arg(
            Arg::new("long")
                .short('l')
                .help("使用长格式列表"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .help("不忽略以'.'开头的条目"),
        )
        .arg(
            Arg::new("human-readable")
                .short('h')
                .help("以人类可读的文件大小显示"),
        )
        .get_matches();

    // 获取命令行参数
    let path = matches.value_of("path").unwrap();
    let show_hidden = matches.is_present("all");
    let show_long_format = matches.is_present("long");
    let show_human_readable = matches.is_present("human-readable");

    // 获取目录项列表
    let mut entries = list_directory(path, show_hidden);

    if show_long_format {
        // 遍历目录项并获取详细信息
        for entry in &mut entries {
            let metadata = fs::metadata(&entry.path).unwrap();

            #[cfg(windows)]
            let permissions = metadata.file_attributes().to_string();
            #[cfg(not(windows))]
            let permissions = format!("{:#o}", metadata.permissions().mode() & 0o7777);

            let modified_time = metadata.modified().unwrap();
            let formatted_time = Local
                .timestamp_opt(modified_time.elapsed().unwrap().as_secs() as i64, 0)
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

/// 目录项结构体
#[derive(Ord, PartialEq, Eq, PartialOrd)]
struct DirectoryEntry {
    file_name: String,
    long_format: String,
    path: std::path::PathBuf,
}

/// 获取目录项列表
fn list_directory(path: &str, show_hidden: bool) -> Vec<DirectoryEntry> {
    let mut entries = fs::read_dir(path)
        .expect(format!("无法打开目录: '{}'", path).as_str())
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

/// 将文件大小转换为人类可读格式
fn human_readable_size(size: u64) -> String {
    let units = [" B", "KB", "MB", "GB", "TB", "PB"];   // 为了对齐, 将单位的宽度设置为 2
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    // FIXME: 将输出对齐, 最长的输出是 1000.0 PB, 需要 9 个字符的宽度
    format!("{:>5.1} {}", size, units[unit_index])
}