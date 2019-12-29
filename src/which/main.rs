use which::which;
use std::env::args;
use std::process::exit;

fn main() {
    let mut count = 0;
    let mut error = 0;

    /* TODO:
       --version
       --help
       --skip-dot
       --all | -a (also where)
    */

    for program_name in args().skip(1) {

        let diag_name = program_name.clone();

        match which(program_name) {
            Ok(path) => {
                let path = path.to_string_lossy();
                println!("{}", path);
            },
            Err(_) => {
                eprintln!("{}: Command not found", diag_name);
                error = 1;
            }
        }

        count += 1;
    }

    if count == 0 {
        /* TODO: this should (maybe) be argv[0] */
        eprintln!("which: Too few arguments");
        exit(1);
    }

    if error == 1 {
        exit(1);
    }
}
