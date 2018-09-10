extern crate clap;
extern crate schedule;

use clap::{App, Arg, SubCommand};

use schedule::*;

fn main() {
    let matches = App::new("schedule")
        .subcommand(
            SubCommand::with_name("list").about("list projects").arg(
                Arg::with_name("item")
                    .help("which item to list")
                    .index(1)
                    .required(false),
            ),
        ).subcommand(
            SubCommand::with_name("create").about("create project").arg(
                Arg::with_name("name")
                    .help("the name of the new project")
                    .index(1)
                    .required(true),
            ),
        ).get_matches();

    init_store();

    match matches.subcommand() {
        ("list", Some(sub)) => match sub.value_of("item") {
            Some(item) => match list_projects().iter().filter(|&x| x.name() == item).next() {
                Some(x) => {
                    println!("{:?}", list_tasks(x));
                }
                _ => panic!("item not found"),
            },
            _ => println!("{:?}", list_projects()),
        },
        ("create", Some(sub)) => println!(
            "{:?}",
            add_project(sub.value_of("name").unwrap().to_string())
        ),
        _ => {}
    }
}
