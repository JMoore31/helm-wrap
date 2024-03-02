mod yaml;
use yaml::{file_to_releases, menu};

use crate::yaml::yaml_to_file;
fn main() {
    let helmfile = file_to_releases();
    let _ = match yaml_to_file(helmfile) {
        Ok(_x) => (),
        Err(error) => panic!("Problem!: {:?}", error),
    };
}
