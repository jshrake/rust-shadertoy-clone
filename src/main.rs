#[macro_use]

extern crate glium;
extern crate clap;
extern crate notify;
extern crate time;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App, SubCommand};
use std::path::Path;
use std::fs::File;
use std::io::Read;

mod new;
mod run;
mod shadertoy_config;

fn main() {
    let app = App::new("shdrs")
        .version("0.1.0")
        .author("Justin Shrake <justinshrake@gmail.com>")
        .about("A native shadertoy environment")
        .subcommand(SubCommand::with_name("new")
            .arg(Arg::with_name("shader_type")
                .help("the shader type to create")
                .index(1)
                .possible_values(&["image"])
                .required(true))
            .arg(Arg::with_name("name")
                .help("the project name")
                .index(2)
                .required(true)))
        .subcommand(SubCommand::with_name("run").arg(Arg::with_name("dir")
            .short("C")
            .long("directory")
            .help("Change to directory before running")
            .takes_value(true)))
        .get_matches();
    let cwd = std::env::current_dir().unwrap();
    match app.subcommand() {
        ("new", Some(sub)) => {
            new::execute(sub.value_of("shader_type").unwrap(),
                         sub.value_of("name").unwrap(),
                         cwd.as_path())
        }
        ("run", Some(sub)) => {
            let dir = sub.value_of("dir").unwrap_or(cwd.to_str().unwrap());
            let project_json_path = Path::new(&dir).join("project.json");
            let mut project_json_file = match File::open(&project_json_path) {
                Err(why) => panic!("Couldn't open{}: {}", project_json_path.display(), why),
                Ok(f) => f,
            };
            let mut project_json_string = String::new();
            match project_json_file.read_to_string(&mut project_json_string) {
                Err(why) => panic!("Couldn't read {}: {}", project_json_path.display(), why),
                Ok(_) => (),
            }
            let project_desrialized: shadertoy_config::Config =
                serde_json::from_str(&project_json_string).unwrap();
            std::env::set_current_dir(dir).unwrap();
            run::execute(project_desrialized);
        }
        _ => unreachable!(),
    }
}
