use std::path::Path;
use async_recursion::async_recursion;
use tokio::fs::{self, File};
use tokio::io::{self};
use futures::future::join_all;

#[tokio::main]
async fn main() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    // 获取当前文件的上一级目录
    let parent_dir = current_dir.parent().unwrap();
    let path = Path::new(parent_dir);
    let mut files = Vec::new();
    read_dir(path, &mut files).await?;
    let mut futures = Vec::new();
    for file in files {
        futures.push(async move {
            let size = get_file_size(&file).await?;
            println!("File path: {:?}, size: {}", file, size);
            Ok::<(), io::Error>(())
        });
    }
    join_all(futures).await;
    Ok(())
}

#[async_recursion]
async fn read_dir(path: &Path, files: &mut Vec<String>) -> io::Result<()> {
    let mut entries = fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            read_dir(&path, files).await?;
        } else {
            let file_path = path.to_string_lossy().to_string();
            files.push(file_path);
        }
    }
    Ok(())
}

async fn get_file_size(file_path: &str) -> io::Result<u64> {
    let file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    Ok(metadata.len())
}
