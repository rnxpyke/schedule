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
                    .get_matches();

    init_store();

    match matches.subcommand_name() {
        Some("list") => {
            println!("{:?}", list_projects());
            },
        _ => {}
    }
}






