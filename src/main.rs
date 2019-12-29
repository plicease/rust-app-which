use which::which;
use std::env::args;
use std::process::exit;

fn main() {
    let mut count = 0;

    for program_name in args().skip(1) {
        let path = which(program_name).unwrap();
        let path = path.to_string_lossy();
        println!("{}", path);
        count += 1;
    }
    
    if count == 0 {
        exit(1);
    }
}
