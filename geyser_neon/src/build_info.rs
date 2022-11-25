build_info::build_info!(fn build_info);

pub fn get_build_info() -> String {
    let build_info = build_info();
    let mut build_info_string = String::new();
    if let Some(version_control) = &build_info.version_control {
        if let Some(git_info) = version_control.git() {
            build_info_string = format!(
                "{}, branch {:?}, commit {}\nbuilt with Rust {}, opt level {}, at {}",
                build_info.crate_info.name,
                git_info
                    .branch
                    .as_ref()
                    .unwrap_or(&"Unknown branch".to_string()),
                git_info.commit_id,
                build_info.compiler.version,
                build_info.optimization_level,
                build_info.timestamp.format("%d/%m/%Y %H:%M %:z"),
            );
        }
    } else {
        build_info_string = format!(
            "{} built with Rust {}, opt level {}, at {}",
            build_info.crate_info.name,
            build_info.compiler.version,
            build_info.optimization_level,
            build_info.timestamp.format("%d/%m/%Y %H:%M %:z"),
        );
    }
    build_info_string
}
