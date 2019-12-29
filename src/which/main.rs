use which::which_in;
use std::env::current_dir;
use std::env::args;
use std::env::var_os;
use std::process::exit;
use std::ffi::OsString;

#[cfg(windows)]
fn get_path(skip_dot: bool) -> OsString {
    if !skip_dot {
        let mut path = OsString::new();
        path.push(current_dir().unwrap().as_os_str());
        path.push(";");
        path.push(var_os("PATH").unwrap());
        return path
    } else {
        return var_os("PATH").unwrap()
    }
}

#[cfg(unix)]
fn get_path(skip_dot: bool) -> OsString {
    var_os("PATH").unwrap()
}

fn main() {
    let mut count = 0;
    let mut error = 0;

    /* TODO:
       --version
       --help
       --skip-dot
       --all | -a (also where)
    */

    let cwd = current_dir().unwrap();
    let path = get_path(false);

    for program_name in args().skip(1) {

        match which_in(program_name.clone(), Some(path.clone()), &cwd) {
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
