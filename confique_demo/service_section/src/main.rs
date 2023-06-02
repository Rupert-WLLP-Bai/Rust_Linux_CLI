// use confique toml feature to read systemd config
// config file path: /etc/systemd/system/sshd.service
// TODO: convert systemd config to toml format
// Example:
// [Unit]
// Description=OpenSSH server daemon
//
// Covert to:
// [Unit]
// Description = "OpenSSH server daemon"

use std::fs;

use confique::Config;
use serde::{Deserialize, Serialize};

// define a struct to store config data
#[derive(Debug, Serialize, Deserialize, Config)]
struct SystemdConfig {
    // Unit section
    #[config(nested)]
    unit: UnitSection,
    // Service section
    #[config(nested)]
    service: ServiceSection,
    // Install section
    #[config(nested)]
    install: InstallSection,
    // Mount section
    #[config(nested)]
    mount: MountSection,
    // Socket section
    #[config(nested)]
    socket: SocketSection,
}

// Unit section
#[derive(Debug, Serialize, Deserialize, Config)]
struct UnitSection {
    description: Option<String>,
    documentation: Option<String>,
}

// Service section
#[derive(Debug, Serialize, Deserialize, Config)]
struct ServiceSection {
    // EnvironmentFile
    environment_file: Option<Vec<String>>,
    // ExecStartPre
    exec_start_pre: Option<Vec<String>>,
    // ExecStart
    exec_start: Option<Vec<String>>,
    // ExecStartPost
    exec_start_post: Option<Vec<String>>,
    // ExecReload
    exec_reload: Option<Vec<String>>,
    // ExecStop
    exec_stop: Option<Vec<String>>,
    // KillMode
    kill_mode: Option<String>,
    // Restart
    restart: Option<String>,
    // RestartPreventExitStatus
    restart_prevent_exit_status: Option<Vec<i32>>,
    // Type
    r#type: Option<String>,
    // RuntimeDirectory
    runtime_directory: Option<String>,
    // RuntimeDirectoryMode
    runtime_directory_mode: Option<String>,
}

// Install section
#[derive(Debug, Serialize, Deserialize, Config)]
struct InstallSection {
    wanted_by: Option<Vec<String>>,
    alias: Option<String>,
}

// Mount section
#[derive(Debug, Serialize, Deserialize, Config)]
struct MountSection {
    what: Option<String>,
    where_: Option<String>,
    type_: Option<String>,
    options: Option<Vec<String>>,
}

// Socket section
#[derive(Debug, Serialize, Deserialize, Config)]
struct SocketSection {
    listen_stream: Option<String>,
    accept: Option<String>,
    service: Option<String>,
}

// function to convert systemd config to toml format
use std::collections::HashMap;

fn convert_systemd_to_toml(systemd: &str) -> String {
    let mut toml = String::new();
    let mut current_section = String::new();
    let mut properties: HashMap<String, Vec<String>> = HashMap::new();

    for line in systemd.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with('[') {
            if !current_section.is_empty() {
                toml.push_str(&current_section);
                toml.push('\n');
                toml.push('\n'); // 添加空行分隔段
            }

            current_section = line.to_owned();
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value = parts[1].trim().to_owned();

        let values = properties.entry(key.to_owned()).or_insert_with(Vec::new);
        values.push(value);
    }

    if !current_section.is_empty() {
        toml.push_str(&current_section);
        toml.push('\n');
        toml.push('\n'); // 添加最后一个段的空行
    }

    for (key, values) in properties {
        toml.push_str(&format!("{} = [", key));

        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                toml.push_str(", ");
            }
            toml.push_str(&format!("\"{}\"", value));
        }

        toml.push_str("]\n");
    }

    toml
}

fn systemd_test_string()->String {
    let systemd = r#"
[Unit]
Description=OpenBSD Secure Shell server
Documentation=man:sshd(8) man:sshd_config(5)
After=network.target auditd.service
ConditionPathExists=!/etc/ssh/sshd_not_to_be_run

[Service]
EnvironmentFile=-/etc/default/ssh
ExecStartPre=/usr/sbin/sshd -t
ExecStart=/usr/sbin/sshd -D $SSHD_OPTS
ExecReload=/usr/sbin/sshd -t
ExecReload=/bin/kill -HUP $MAINPID
KillMode=process
Restart=on-failure
RestartPreventExitStatus=255
Type=notify
RuntimeDirectory=sshd
RuntimeDirectoryMode=0755

[Install]
WantedBy=multi-user.target
Alias=sshd.service
"#;
    systemd.to_owned()
}

fn main() {
    // read systemd config
    let systemd = systemd_test_string();
    // convert systemd config to toml format
    let toml = convert_systemd_to_toml(&systemd);
    // print toml format
    println!("{}", toml);
}
