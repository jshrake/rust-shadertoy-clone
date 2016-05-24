#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#[macro_use]

extern crate glium;
extern crate clap;
extern crate notify;
extern crate time;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App, SubCommand};

mod new;
mod run;
mod shadertoy_config;

fn main() {
    let app = App::new("shdrs")
                  .version("0.1.0")
                  .author("Justin Shrake <justinshrake@gmail.com>")
                  .about("A native shadertoy environment")
                  .subcommand(SubCommand::with_name("new")
                                  .arg(Arg::with_name("type")
                                           .help("the project type to create")
                                           .index(1)
                                           .possible_values(&["image"])
                                           .required(true))
                                  .arg(Arg::with_name("name")
                                           .help("the project name")
                                           .index(2)
                                           .required(true)))
                  .subcommand(SubCommand::with_name("run"))
                  .get_matches();
    let fragment_shader_path = std::path::Path::new("shader.frag");
    match app.subcommand() {
        ("new", Some(sub)) => {
            new::execute(sub.value_of("type").unwrap(), sub.value_of("name").unwrap())
        }
        ("run", Some(_)) => run::execute(&fragment_shader_path),
        _ => unreachable!(),
    }
}
