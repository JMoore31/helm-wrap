
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::io::{BufRead, BufReader,Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Releases {
    pub releases: Vec<Release>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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


pub fn file_to_releases()-> Releases{
    let file = std::fs::OpenOptions::new().read(true).open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut releases = Releases::new();
    let mut release = Release::default();
    for line in reader.lines(){
        let split = line.unwrap();
        let split = split.split_whitespace();
        let pair = split.collect::<Vec<&str>>();
        match pair[0] {
            "name" => release.name = (pair[1]).to_string(),
            "chart" => release.chart = (pair[1]).to_string(),
            "namespace" => release.namespace = (pair[1]).to_string(),
            _ => println!("Unidentified key in pair"),
        }
    }
    releases.add_release(release);
    releases
}
