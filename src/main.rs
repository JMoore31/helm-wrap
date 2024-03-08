mod yaml;
use yaml::input_to_helmfile;
use crate::yaml::{file_to_helmfile, to_file};
fn main() {
    println!("Press 1 for file input, and 2 for manual CLI");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("failed");
    let val: usize = line.trim().parse().expect("Was not a number!!!");
    
    let helmfile = match val {
        1 => file_to_helmfile(),
        2 => input_to_helmfile(),
        _ => panic!("Not a number..."),
        
    };
    to_file(&helmfile);
}
