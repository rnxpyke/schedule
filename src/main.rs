extern crate clap;
extern crate schedule;

use clap::{App, Arg, SubCommand};

use schedule::*;

use std::io;

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
        ).subcommand(
            SubCommand::with_name("add")
                .about("add task to project")
                .arg(
                    Arg::with_name("project")
                        .help("the name of the project")
                        .index(1)
                        .required(true),
                ).arg(
                    Arg::with_name("task")
                        .help("the name of the new task")
                        .index(2)
                        .required(true),
                ),
        ).subcommand(
            SubCommand::with_name("remove")
                .about("remove task or project")
                .arg(
                    Arg::with_name("project")
                        .help("the name of the project")
                        .index(1)
                        .required(true),
                ).arg(Arg::with_name("task").help("the name of the task").index(2)),
        ).get_matches();

    init_store();

    match matches.subcommand() {
        ("list", Some(sub)) => match sub.value_of("item") {
            Some(item) => match get_project(item.to_string()) {
                Some(x) => {
                    println!("{:?}", list_tasks(&x));
                }
                _ => panic!("item not found"),
            },
            _ => println!("{:?}", list_projects()),
        },
        ("create", Some(sub)) => println!(
            "{:?}",
            add_project(sub.value_of("name").unwrap().to_string())
        ),
        ("add", Some(sub)) => {
            let project = sub.value_of("project").unwrap().to_string();
            let task = sub.value_of("task").unwrap().to_string();
            println!("{:?}", add_task_to_project(project, task));
        }
        ("remove", Some(sub)) => {
            println!("{:?}", remove(sub));
        }
        _ => {}
    }
}

fn remove(sub: &clap::ArgMatches) -> io::Result<()> {
    let project_name = sub.value_of("project").unwrap().to_string();
    let project = wrap_option(get_project(project_name), "project not found".to_string())?;
    match sub.value_of("task") {
        //todo
        Some(s) => Err(io::Error::new(io::ErrorKind::Other, "not implemented")),
        None => project.remove(),
    }
}

fn add_task_to_project(name: String, task: String) -> io::Result<()> {
    let project = wrap_option(get_project(name), "project not found".to_string())?;
    add_tasks_err(&project, &TaskInfo::from_name(task))?;
    Ok(())
}

#[inline]
fn wrap_option<A>(o: Option<A>, msg: String) -> io::Result<A> {
    o.ok_or(io::Error::new(io::ErrorKind::Other, msg))
}

fn get_project(name: String) -> Option<Project> {
    list_projects()
        .into_iter()
        .filter(|x| x.name() == name)
        .next()
}
