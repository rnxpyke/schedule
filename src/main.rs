extern crate clap;
extern crate schedule;


use clap::{App,Arg,SubCommand};

use schedule::*;

use std::fs;
use std::env;


fn main() {
    let matches = App::new("schedule")
                    .subcommand(SubCommand::with_name("list")
                        .about("list projects"))
                    .subcommand(SubCommand::with_name("create")
                        .about("create project")
                        .arg(Arg::with_name("name")
                                .help("the name of the new project")
                                .index(1)
                                .required(true)))
                    .get_matches();

    init_store();

    match matches.subcommand_name() {
        Some("list") => {
            println!("{:?}", list_projects());
            },
        Some("create") => { 
            println!("{:?}", add_project(
                matches.subcommand_matches("create")
                .unwrap()
                .value_of("name")
                .unwrap()
                .to_string())); },
        _ => {}
    }
}






