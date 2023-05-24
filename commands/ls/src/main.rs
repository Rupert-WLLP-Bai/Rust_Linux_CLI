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
//! 版本：0.1.1
//! 更新记录: 加入了颜色输出功能

use chrono::prelude::*;
use chrono::Local;
use clap::{App, Arg};
use colored::*;
use std::fs;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

/// 主函数
pub fn main() {
    // 创建命令行解析器
    let matches = App::new("ls")
        .arg(Arg::new("path").default_value(".").index(1))
        .arg(Arg::new("long").short('l').help("使用长格式列表"))
        .arg(Arg::new("all").short('a').help("不忽略以'.'开头的条目"))
        .arg(
            Arg::new("human-readable")
                .short('h')
                .help("以人类可读的文件大小显示"),
        )
        .arg(Arg::new("color").long("color").help("启用颜色输出"))
        .get_matches();

    // 获取命令行参数
    let path = matches.value_of("path").unwrap();
    let show_hidden = matches.is_present("all");
    let show_long_format = matches.is_present("long");
    let show_human_readable = matches.is_present("human-readable");
    let use_color = matches.is_present("color");

    // 获取目录项列表
    let mut entries = list_directory(path, show_hidden);

    if show_long_format {
        // 遍历目录项并获取详细信息
        let mut max_permissions = 0;
        let mut max_size = 0;

        for entry in &mut entries {
            let metadata = fs::metadata(&entry.path).unwrap();

            #[cfg(windows)]
            let permissions = metadata.file_attributes().to_string();
            #[cfg(not(windows))]
            let permissions = format!("{:#o}", metadata.permissions().mode() & 0o7777);
            entry.permissions = permissions.clone();

            let modified_time = metadata.modified().unwrap();
            let formatted_time = Local
                .timestamp_opt(modified_time.elapsed().unwrap().as_secs() as i64, 0)
                .unwrap()
                .format("%b %e %H:%M")
                .to_string();
            entry.formatted_time = formatted_time.clone();

            let size = metadata.len();
            entry.size = size;

            if permissions.len() > max_permissions {
                max_permissions = permissions.len();
            }
            if size.to_string().len() > max_size {
                max_size = size.to_string().len();
            }
        }

        for entry in &mut entries {
            entry.permissions = format!("{:width$}", entry.permissions, width = max_permissions);
            entry.formatted_time = format!("{:width$}", entry.formatted_time, width = 12);
            entry.size_string = if show_human_readable {
                human_readable_size(entry.size)
            } else {
                entry.size.to_string()
            };
            entry.size_string = format!("{:width$}", entry.size_string, width = max_size);
            entry.long_format = format!(
                "{} {} {} {}",
                entry.permissions, entry.formatted_time, entry.size_string, entry.file_name
            );
        }
    }

    entries.sort();

    // 打印目录项列表
    for entry in entries {
        if show_long_format {
            if use_color {
                let file_type = fs::metadata(&entry.path)
                    .map(|metadata| metadata.file_type().is_dir())
                    .unwrap_or(false);

                let permissions = if file_type {
                    entry.permissions.blue().to_string()
                } else {
                    entry.permissions.red().to_string()
                };

                let formatted_time = entry.formatted_time.green().to_string();
                let size_string = entry.size_string.yellow().to_string();
                let file_name = entry.file_name.bright_white().to_string();

                let long_format = format!(
                    "{} {} {} {}",
                    permissions, formatted_time, size_string, file_name
                );

                println!("{}", long_format);
            } else {
                println!("{}", entry.long_format);
            }
        } else {
            if use_color {
                let file_type = fs::metadata(&entry.path)
                    .map(|metadata| metadata.file_type().is_dir())
                    .unwrap_or(false);

                let file_name = if file_type {
                    entry.file_name.blue().bold().to_string()
                } else {
                    entry.file_name.bright_white().to_string()
                };

                println!("{}", file_name);
            } else {
                println!("{}", entry.file_name);
            }
        }
    }
}

/// 目录项结构体
#[derive(Ord, PartialEq, Eq, PartialOrd)]
pub struct DirectoryEntry {
    file_name: String,
    long_format: String,
    path: std::path::PathBuf,
    permissions: String,
    formatted_time: String,
    size: u64,
    size_string: String,
}

/// 获取目录项列表
pub fn list_directory(path: &str, show_hidden: bool) -> Vec<DirectoryEntry> {
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
                permissions: "".to_owned(),
                formatted_time: "".to_owned(),
                size: 0,
                size_string: "".to_owned(),
            })
        })
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    entries.sort_by_key(|entry| entry.file_name.clone());

    entries
}

/// 将文件大小转换为人类可读格式
pub fn human_readable_size(size: u64) -> String {
    let units = [" B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:>5.1} {}", size, units[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_list_directory() {
        // 删除测试用的文件夹(如果存在的话)
        // 输出warning信息，可能上一次测试没有成功删除
        if fs::metadata("ls_test").is_ok() {
            println!("warning: 上一次测试没有成功删除测试用的文件夹");
            let remove_result = fs::remove_dir_all("ls_test");
            assert!(remove_result.is_ok());
        }

        // 创建测试用的文件夹
        fs::create_dir("ls_test").unwrap();

        // 在"ls_test"文件夹中创建一个文件
        fs::File::create("ls_test/test_file.txt").unwrap();

        // 测试list_directory函数的输出是否正确
        let path = "ls_test";
        let show_hidden = false;
        let entries = list_directory(path, show_hidden);

        // 检查目录项数量是否与预期一致
        assert_eq!(entries.len(), 1);

        // 检查目录项的文件名是否正确
        assert_eq!(entries[0].file_name, "test_file.txt");

        // 检查目录项的路径是否正确(注意在Windows上的路径分隔符为'\\')
        #[cfg(windows)]
        assert_eq!(entries[0].path.to_str().unwrap(), "ls_test\\test_file.txt");
        #[cfg(not(windows))]
        assert_eq!(entries[0].path.to_str().unwrap(), "ls_test/test_file.txt");

        // 删除测试用的文件夹(注意检查是否成功删除)
        let remove_result = fs::remove_dir_all("ls_test");
        assert!(remove_result.is_ok());

    }

    #[test]
    fn test_human_readable_size() {
        // 测试human_readable_size函数的输出是否正确
        assert_eq!(human_readable_size(0), "  0.0  B");
        assert_eq!(human_readable_size(1024), "  1.0 KB");
        assert_eq!(human_readable_size(2048), "  2.0 KB");
        assert_eq!(human_readable_size(1048576), "  1.0 MB");
        assert_eq!(human_readable_size(15728640), " 15.0 MB");
        assert_eq!(human_readable_size(1073741824), "  1.0 GB");
    }

    // TODO: 编写更多的测试用例
    // TODO: 将测试用例移动到单独的文件中
}
