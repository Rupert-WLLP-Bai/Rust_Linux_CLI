use ini::ini;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct UnitSection {
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub wants: Option<String>,
    pub requires: Option<String>,
    pub requisite: Option<String>,
    pub binds_to: Option<String>,
    pub part_of: Option<String>,
    pub upholds: Option<String>,
    pub conflicts: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub on_failure: Option<String>,
    pub on_success: Option<String>,
    pub propagates_reload_to: Option<String>,
    pub reload_propagated_from: Option<String>,
    pub propagates_stop_to: Option<String>,
    pub stop_propagated_from: Option<String>,
    pub joins_namespace_of: Option<String>,
    pub requires_mounts_for: Option<String>,
    pub on_failure_job_mode: Option<String>,
    pub ignore_on_isolate: Option<bool>,
    pub stop_when_unneeded: Option<bool>,
    pub refuse_manual_start: Option<bool>,
    pub refuse_manual_stop: Option<bool>,
    pub allow_isolate: Option<bool>,
    pub default_dependencies: Option<bool>,
    pub collect_mode: Option<String>,
    pub failure_action: Option<String>,
    pub success_action: Option<String>,
}

// TODO: 读取配置文件
// 参考: https://docs.rs/ini/1.3.0/ini/
fn parse_unit_section(path: &str) -> Result<UnitSection, Box<dyn std::error::Error>> {
    let conf = ini!(path); // 读取配置文件
    let mut unit_section = UnitSection::default(); // 初始化结构体
    let unit_section_map = conf.get("unit").ok_or("Missing 'Unit' section")?; // 获取'Unit'部分

    // 将map中的值赋值给结构体
    unit_section.description = unit_section_map.get("description").cloned().flatten();
    unit_section.documentation = unit_section_map.get("documentation").cloned().flatten();
    unit_section.wants = unit_section_map.get("wants").cloned().flatten();
    unit_section.requires = unit_section_map.get("requires").cloned().flatten();
    unit_section.requisite = unit_section_map.get("requisite").cloned().flatten();
    unit_section.binds_to = unit_section_map.get("binds_to").cloned().flatten();
    unit_section.part_of = unit_section_map.get("part_of").cloned().flatten();
    unit_section.upholds = unit_section_map.get("upholds").cloned().flatten();
    unit_section.conflicts = unit_section_map.get("conflicts").cloned().flatten();
    unit_section.before = unit_section_map.get("before").cloned().flatten();
    unit_section.after = unit_section_map.get("after").cloned().flatten();
    unit_section.on_failure = unit_section_map.get("on_failure").cloned().flatten();
    unit_section.on_success = unit_section_map.get("on_success").cloned().flatten();
    unit_section.propagates_reload_to = unit_section_map.get("propagates_reload_to").cloned().flatten();
    unit_section.reload_propagated_from = unit_section_map.get("reload_propagated_from").cloned().flatten();
    unit_section.propagates_stop_to = unit_section_map.get("propagates_stop_to").cloned().flatten();
    unit_section.stop_propagated_from = unit_section_map.get("stop_propagated_from").cloned().flatten();
    unit_section.joins_namespace_of = unit_section_map.get("joins_namespace_of").cloned().flatten();
    unit_section.requires_mounts_for = unit_section_map.get("requires_mounts_for").cloned().flatten();
    unit_section.on_failure_job_mode = unit_section_map.get("on_failure_job_mode").cloned().flatten();
    unit_section.ignore_on_isolate = unit_section_map.get("ignore_on_isolate").cloned().flatten().map(|x| x.parse::<bool>().unwrap());
    unit_section.stop_when_unneeded = unit_section_map.get("stop_when_unneeded").cloned().flatten().map(|x| x.parse::<bool>().unwrap());
    unit_section.refuse_manual_start = unit_section_map.get("refuse_manual_start").cloned().flatten().map(|x| x.parse::<bool>().unwrap());
    unit_section.refuse_manual_stop = unit_section_map.get("refuse_manual_stop").cloned().flatten().map(|x| x.parse::<bool>().unwrap());
    unit_section.allow_isolate = unit_section_map.get("allow_isolate").cloned().flatten().map(|x| x.parse::<bool>().unwrap());
    unit_section.default_dependencies = unit_section_map.get("default_dependencies").cloned().flatten().map(|x| x.parse::<bool>().unwrap());
    unit_section.collect_mode = unit_section_map.get("collect_mode").cloned().flatten();
    unit_section.failure_action = unit_section_map.get("failure_action").cloned().flatten();
    unit_section.success_action = unit_section_map.get("success_action").cloned().flatten();


    Ok(unit_section)
}

fn main() {
    match parse_unit_section("/etc/systemd/system/sshd.service") {
        Ok(unit_section) => {
            println!("{:#?}", unit_section);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
