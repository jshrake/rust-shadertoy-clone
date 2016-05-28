use serde_json;
use shadertoy_config;
use std;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn new_image_shader(name: &str, project_parent_dir: &Path) {
    // Make sure the project_dir path exists
    let project_dir = Path::new(project_parent_dir).join(name);
    if !project_dir.exists() {
        std::fs::create_dir_all(&project_dir).unwrap();
    }
    // Read in the project.json image shader template, make some changes
    let project_template = include_str!("templates/image/project.json");
    let mut project_deserialized: shadertoy_config::Config = serde_json::from_str(project_template)
        .unwrap();
    project_deserialized.shader.info.name = name.to_string();
    project_deserialized.shader.info.author = "AUTHOR".to_string();
    // Build the path: project_dir/project.json
    let project_json_path = project_dir.join("project.json");
    // Create project_json_path and serialize
    let mut project_json_file = match File::create(&project_json_path) {
        Err(why) => panic!("Couldn't create {}: {}", project_json_path.display(), why),
        Ok(f) => f,
    };
    let project_serialized = serde_json::to_string_pretty(&project_deserialized).unwrap();
    match project_json_file.write_all(project_serialized.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", project_json_path.display(), why),
        Ok(_) => (),
    }
    // Now write the shader template
    let shader_path = project_dir.join("image.glsl");
    let shader_template = include_str!("templates/image/image.glsl");
    let mut shader_file = match File::create(&shader_path) {
        Err(why) => panic!("Couldn't create {}: {}", shader_path.display(), why),
        Ok(f) => f,
    };
    match shader_file.write_all(shader_template.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", shader_path.display(), why),
        Ok(_) => (),
    }
}

pub fn execute(kind: &str, name: &str, project_parent_dir: &Path) {
    match kind {
        "image" => new_image_shader(name, project_parent_dir),
        _ => unreachable!(),
    }
}
