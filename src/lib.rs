use std::fs;
use std::io;
use std::io::Write;
use std::env;
use std::path::PathBuf;


pub fn get_path_string() -> String {
    let key = "HOME";
    let home = match env::var(key) {
        Ok(x) => x,
        Err(e) => panic!(e)
    };
    home + "/.schedule"
}

pub fn get_path() -> PathBuf {
    PathBuf::from(get_path_string())
}

#[allow(unused_must_use)]
pub fn init_store() {
    fs::create_dir(get_path());
}

#[derive(Debug, Clone)]
pub struct Project {
    path: PathBuf
}

#[derive(Debug, Clone)]
pub struct Task {
    path: PathBuf
}

impl Project {
    fn from(path: PathBuf) -> Project {
        Project {
            path
        }
    }
}

pub fn list_projects() -> Vec<Project> {
    match list_projects_res() {
        Ok(x) => x,
        _ => vec!()
    }
}

fn list_projects_res() -> io::Result<Vec<Project>> {
    let dir = get_path();
    
    let mut projects: Vec<Project> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            projects.push(Project::from(path));
        }
    }
    Ok(projects)
}

pub fn add_project(name: String) -> io::Result<()> {
    let mut path = get_path();

    //create project dir
    path.push(&name);
    fs::create_dir(&path)?;

    //create and write name file 
    path.push("name");
    let mut file = fs::File::create(&path)?;
    file.write(name.as_bytes())?;
    path.pop();

    //create tasks dir
    path.push("tasks");
    fs::create_dir(&path)?;
    Ok(())
}


fn list_tasks_res(p: &Project) -> io::Result<Vec<Task>> {
    let mut dir = p.path.clone();
    dir.push("tasks");
    let mut tasks: Vec<Task> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            tasks.push(Task{path});
        }
    }
    Ok(tasks)
}

pub fn list_tasks(p: &Project) -> Vec<Task> {
    match list_tasks_res(p) {
        Ok(x) => x,
        _ => vec!()
    }
}

#[derive(Copy,Clone,Debug)]
pub enum Group {
    Todo,
    InProgress,
    Finished,
    Misc
}

#[derive(Clone,Debug)]
pub struct TaskInfo {
    name: String,
    group: Group,
}

pub fn add_tasks_err(p: &Project, task: &TaskInfo) -> io::Result<()> {
    let mut dir = p.path.clone();
    dir.push("tasks");
    dir.push(task.name.clone());
    fs::create_dir(&dir)?;
    
    dir.push("name");
    let mut name = fs::File::create(&dir)?;
    name.write_all(task.name.as_bytes())?;
    
    dir.pop();
    dir.push("groups");

    let mut groups = fs::File::create(&dir)?;
    groups.write_all(format!("{:?}", &task.group).as_bytes())?;

    Ok(())
}


