mod yaml;

use yaml::{file_to_releases,Repositories, Repository};

use crate::yaml::yaml_to_file;
fn main() {
    let release_list = file_to_releases();
    let mut repository_list = Repositories::new();

    let bitnami = Repository {
        name: "bitnami".to_string(),
        url: "https://charts.bitnami.com/bitnami".to_string(),
    };
    repository_list.add_repository(bitnami);
    let _ = match yaml_to_file(release_list, repository_list) {
        Ok(_x) => (),
        Err(error) => panic!("Problem!: {:?}", error),
    };
}
