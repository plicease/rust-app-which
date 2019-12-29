use which::which_in;
use std::env::current_dir;
use std::env::args;
use std::env::var_os;
use std::process::exit;
use std::ffi::OsString;
use getopts::Options;

struct App {
    program_name: String,
    skip_dot: bool,
    command_vec: Vec<String>
}

impl App {

    fn new() -> App {
        App {
            program_name: String::from(""),
            skip_dot: false,
            command_vec: [].to_vec(),
        }
    }

    #[cfg(windows)]
    fn get_path(&self) -> OsString {
        if !self.skip_dot {
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
    fn get_path(&self) -> OsString {
        var_os("PATH").unwrap()
    }

    fn print_usage(&self, opts: Options, message: Option<getopts::Fail>) -> ! {
        let brief = format!("Usage: {} [options] COMMAND", self.program_name);

        match message {
            Some(message) => {
                eprintln!("{}: {}", self.program_name, message);
                eprint!("{}", opts.usage(&brief));
                exit(1)
            },
            None => {
                print!("{}", opts.usage(&brief));
                exit(0)
            }
        }
    }

    fn get_options(&mut self) {
        let args: Vec<String> = args().collect();
        self.program_name = args[0].clone();

        let mut opts = Options::new();

        /* TODO: implement all */
        opts.optflag("a","all","Print all matches in PATH, not just the first.");
        opts.optflag("","help", "Print this help and exit successfully.");
        #[cfg(windows)]
        opts.optflag("","skip-dot", "Do not search current directory (a la PowerShell)");
        opts.optflag("v","version", "Print version and exit successfully.");

        let matches = match opts.parse(&args[1..]) {
            Ok(m)  => m,
            Err(f) => self.print_usage(opts, Some(f))
        };

        if matches.opt_present("help") {
            self.print_usage(opts, None)
        }

        if matches.opt_present("version") {
            println!("Rusty which v1.00, Copyright (c) 2019 Graham Ollis.");
            exit(0)
        }

        if matches.free.is_empty() {
            eprintln!("{}: Too few arguments", self.program_name);
            exit(1)
        }

        if matches.opt_defined("skip-dot") && matches.opt_present("skip-dot") {
            self.skip_dot = true
        }

        self.command_vec = matches.free;
    }

    fn run(&self) -> i32 {
        let mut rv = 0;
        let cwd = current_dir().unwrap();
        let path = self.get_path();

        for program_name in self.command_vec.iter() {

            match which_in(program_name.clone(), Some(path.clone()), &cwd) {
                Ok(path) => {
                    let path = path.to_string_lossy();
                    println!("{}", path);
                },
                Err(_) => {
                    eprintln!("{}: Command not found", program_name);
                    rv = 1;
                }
            }
        }

        rv
    }

}

fn main() {
    let mut app = App::new();
    app.get_options();
    let rv = app.run();
    exit(rv);
}
