extern crate sysinfo;
extern crate winapi;

// 先使用sysinfo实现一个最简单的ps命令
// 之后再使用winapi实现一个更加完整的ps命令

use sysinfo::{ProcessExt, System, SystemExt};


fn main() {
    let s = System::new_all();

    // 打印表头
    println!("{:<6} {:<6} {:<6} {:<10} {:<30} {:<30}", "PID", "CPU%", "MEM%", "CMD", "EXE PATH", "ROOT PATH");

    // 打印每个进程的信息
    for (pid, process) in s.processes(){
        println!("{:<6} {:<6.2} {:<6.2} {:<10} {:<30} {:<30}", 
            pid,
            process.cpu_usage(),
            process.memory() as f32 / s.total_memory() as f32 * 100.0,
            process.name(),
            process.exe().to_str().unwrap_or(""),
            process.root().to_str().unwrap_or(""));
    }
}
