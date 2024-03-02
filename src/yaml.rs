use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::{io::{BufRead, BufReader,Write}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Releases {
    pub releases: Vec<Release>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Release {
    pub name: String,
    pub chart: String,
    pub namespace: String,
    pub needs: Vec<String>,
    pub values: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Helmfile {
    repositories: Repositories,
    releases: Releases,
}

impl Helmfile {
    pub fn new() -> Self{
        Helmfile {
            releases: Releases::new(),
            repositories: Repositories::new(),
        }
    }
    fn cli_add_release(&mut self){
        todo!();
    }
    fn cli_add_repository(&mut self){
        todo!();
    }
    fn print_releases(&mut self){
        todo!();
    }
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
pub fn yaml_to_file(helmfile: Helmfile) -> Result<(), serde_yaml::Error> {
    let separator = "---\n".as_bytes();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("helmfile.yaml")
        .expect("Could not open new file");
    serde_yaml::to_writer(&file, &helmfile.repositories).unwrap();
    let _ = &file.write_all(separator);
    serde_yaml::to_writer(&file, &helmfile.releases).unwrap();
    Ok(())
}
pub fn menu() -> String {
    println!("-----------------");
    println!("Helm-Wrap Menu\n1) Add a new release\n2) Add a Repository\n3) Print Releases\n4) Finalise choices");
    println!("-----------------");
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

pub fn input_to_helmfile()-> Helmfile {
    let mut helmfile = Helmfile::new();
    match menu().as_ref() {
        "1" => helmfile.cli_add_release(),
        "2" => helmfile.cli_add_repository(),
        "3" => helmfile.print_releases(),
        "4" => (),
        _ => {
            println!("Not a valid input, try again");

        }
    }
    let mut releases = Releases::new();
    helmfile
}
pub fn file_to_releases()-> Helmfile{
    let file = std::fs::OpenOptions::new().read(true).open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut releases = Releases::new();
    let mut release = Release::default();
    let mut repositories = Repositories::new();
    for line in reader.lines() {
        let split = line.unwrap();
        let split = split.split_whitespace();
        let pair = split.collect::<Vec<&str>>();
        match pair[0] {
            "name" => release.name = (pair[1]).to_string(),
            "chart" => release.chart = (pair[1]).to_string(),
            "namespace" => release.namespace = (pair[1]).to_string(),
            "needs" => {
                for n in 1..pair.len() {
                    release.needs.push(pair[n].to_string());
                }
            }
            "values" => {
                for n in 1..pair.len() {
                    release.values.push(pair[n].to_string());
                }
            }
            "repository" => repositories.add_repository(Repository {
                name: pair[1].to_string(),
                url: pair[2].to_string(),
            }),
            "---" => {
                releases.add_release(release);
                release = Release::default();
            }
            _ => println!("Unidentified key in pair"),
        }
    }

    println!("{:?}", releases);
    let helmfile = Helmfile {
        releases: releases,
        repositories: repositories,
    };
    helmfile
}
