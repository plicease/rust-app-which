use which::which;
use std::env::args;
use std::process::exit;

fn main() {
    let mut count = 0;
    let mut error = 0;

    /* TODO:
       --version
       --help
       --skip-dot  /* FIXME currently we don't search . */
       --all | -a (also where)
    */

    for program_name in args().skip(1) {

        match which(program_name.clone()) {
            Ok(path) => {
                let path = path.to_string_lossy();
                println!("{}", path);
            },
            Err(_) => {
                eprintln!("{}: Command not found", program_name);
                error = 1;
            }
        }

        count += 1;
    }

    if count == 0 {
        /* TODO: this should (maybe) be using argv[0] instead of 'which' */
        eprintln!("which: Too few arguments");
        exit(1);
    }

    if error == 1 {
        exit(1);
    }
}
