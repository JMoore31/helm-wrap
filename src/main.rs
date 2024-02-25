mod yaml;
use yaml::{Release, Releases, Repositories, Repository};

use crate::yaml::yaml_to_file;
fn main() {
    let mut release_list = Releases::new();
    let mut repository_list = Repositories::new();

    let bitnami = Repository {
        name: "bitnami".to_string(),
        url: "https://charts.bitnami.com/bitnami".to_string(),
    };

    let kafka = Release {
        name: "kafka".to_string(),
        chart: "bitnami/kafka".to_string(),
        namespace: "kafka".to_string(),
    };
    repository_list.add_repository(bitnami);
    release_list.add_release(kafka);
    let _ = match yaml_to_file(release_list, repository_list) {
        Ok(_x) => (),
        Err(error) => panic!("Problem!: {:?}", error),
    };
}
