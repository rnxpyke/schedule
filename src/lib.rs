use std::fs;
use std::io;
use std::io::Write;
use std::env;
use std::path::PathBuf;


pub fn get_path() -> String {
    let key = "HOME";
    let home = match env::var(key) {
        Ok(x) => x,
        Err(e) => panic!(e)
    };
    home + "/.schedule"
}

pub fn init_store() {
    fs::create_dir(get_path());
}

#[derive(Debug, Clone)]
pub struct Project {
    path: PathBuf
}

impl Project {
    pub fn from(path: PathBuf) -> Project {
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
    let dir = get_path();
    fs::create_dir(dir.clone()+"/"+&name)?;
    let mut file = fs::File::create(dir+"/"+&name+"/name")?;
    file.write(name.as_bytes())?;
    Ok(())
}


