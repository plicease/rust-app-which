use which::which_in;
use std::env::current_dir;
use std::env::args;
use std::env::var_os;
use std::process::exit;
use std::ffi::OsString;
use getopts::Options;

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

fn print_usage(program: &str, opts: Options, message: Option<getopts::Fail>) -> ! {
    let brief = format!("Usage: {} [options] COMMAND", program);

    match message {
        Some(message) => {
            eprintln!("{}: {}", program, message);
            eprint!("{}", opts.usage(&brief));
            exit(1);
        },
        None => {
            print!("{}", opts.usage(&brief));
            exit(0);
        }
    }
}

fn get_options() -> getopts::Matches {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    /* TODO: implement all */
    opts.optflag("a","all","Print all matches in PATH, not just the first.");
    opts.optflag("","help", "Print this help and exit successfully.");
    #[cfg(windows)]
    opts.optflag("","skip-dot", "Do not search current directory (a la PowerShell)");
    opts.optflag("v","version", "Print version and exit successfully.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(f) => print_usage(&program, opts, Some(f))
    };

    if matches.opt_present("help") {
        print_usage(&program, opts, None);
    }

    if matches.opt_present("version") {
        println!("Rusty which v1.00, Copyright (c) 2019 Graham Ollis.");
        exit(0);
    }

    if matches.free.is_empty() {
        eprintln!("{}: Too few arguments", program);
        exit(1);
    }

    return matches
}

fn main() {
    let mut error = 0;

    let m = get_options();

    let cwd = current_dir().unwrap();
    let path = get_path(m.opt_present("skip-dot"));

    for program_name in m.free {

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
    }

    if error == 1 {
        exit(1);
    }
}
