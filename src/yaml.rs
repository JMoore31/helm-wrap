use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Releases {
    pub releases: Vec<Release>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub name: String,
    pub chart: String,
    pub namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repositories {
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
}

impl Releases {
    pub fn new() -> Self {
        Releases {
            releases: Vec::new(),
        }
    }
    pub fn add_release(&mut self, release: Release) {
        self.releases.push(release);
    }
}

impl Repositories {
    pub fn new() -> Self {
        Repositories {
            repositories: Vec::new(),
        }
    }
    pub fn add_repository(&mut self, repository: Repository) {
        self.repositories.push(repository);
    }
}
pub fn yaml_to_file(
    release_list: Releases,
    repository_list: Repositories,
) -> Result<(), serde_yaml::Error> {
    let separator = "---\n".as_bytes();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("helmfile.yaml")
        .expect("Could not open new file");
    serde_yaml::to_writer(&file, &repository_list).unwrap();
    let _ = &file.write_all(separator);
    serde_yaml::to_writer(&file, &release_list).unwrap();
    Ok(())
}
