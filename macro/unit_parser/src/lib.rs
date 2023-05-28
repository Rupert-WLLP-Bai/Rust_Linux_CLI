extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Unit)]
pub fn unit_derive(input: TokenStream) -> TokenStream {
    // 解析TokenStream为AST
    let ast = parse_macro_input!(input as DeriveInput);

    // 构建新的TokenStream
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn parse_unit_section(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
                let conf: std::collections::HashMap<String, std::collections::HashMap<String, Option<String>>> = ini!(path); // 读取配置文件
                let mut unit_section: Self = Self::default(); // 初始化结构体
                let unit_section_map: &std::collections::HashMap<String, Option<String>> = conf.get("unit").ok_or("Missing 'Unit' section")?; // 获取'Unit'部分

                // 将map中的值赋值给结构体
                // 这里你需要根据你的结构体的字段来添加代码
                // 例如：
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
                unit_section.ignore_on_isolate = unit_section_map.get("ignore_on_isolate").cloned().flatten().map(|x: String| x.parse::<bool>().unwrap());
                unit_section.stop_when_unneeded = unit_section_map.get("stop_when_unneeded").cloned().flatten().map(|x: String| x.parse::<bool>().unwrap());
                unit_section.refuse_manual_start = unit_section_map.get("refuse_manual_start").cloned().flatten().map(|x: String| x.parse::<bool>().unwrap());
                unit_section.refuse_manual_stop = unit_section_map.get("refuse_manual_stop").cloned().flatten().map(|x: String| x.parse::<bool>().unwrap());
                unit_section.allow_isolate = unit_section_map.get("allow_isolate").cloned().flatten().map(|x: String| x.parse::<bool>().unwrap());
                unit_section.default_dependencies = unit_section_map.get("default_dependencies").cloned().flatten().map(|x: String| x.parse::<bool>().unwrap());
                unit_section.collect_mode = unit_section_map.get("collect_mode").cloned().flatten();
                unit_section.failure_action = unit_section_map.get("failure_action").cloned().flatten();
                unit_section.success_action = unit_section_map.get("success_action").cloned().flatten();
                Ok(unit_section)
            }
        }
    };

    // 返回生成的TokenStream
    gen.into()
}
