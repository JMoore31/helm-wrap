mod yaml;
use yaml::{input_to_helmfile, menu};

use crate::yaml::yaml_to_file;
fn main() {
    let helmfile = input_to_helmfile();
    let _ = match yaml_to_file(helmfile) {
        Ok(_x) => (),
        Err(error) => panic!("Problem!: {:?}", error),
    };
}
