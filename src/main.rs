use clap::App;
use clap::Arg;
use log::info;
use std::process::Command;
use which::which;

fn main() {
    let matches = App::new("resoom")
        .arg(Arg::with_name("file").required(true))
        .get_matches();
    if let Some(file) = matches.value_of("file") {
        info!("file: {}", file);
        compile(check_path().expect("compiler not found"), file, ".");
    }
}

/// Constructs a new `Command` from the first program from `compilers.txt` found
/// by `which`
fn check_path() -> Option<Command> {
    match include_str!("compilers.txt")
        .lines()
        .find(|c| which(c).is_ok())
    {
        Some(c) => Some(Command::new(c)),
        None => {
            info!("no compiler found locally");
            None
        }
    }
}

fn compile(mut compiler: Command, source: &str, current_dir: &str) {
    compiler
        .current_dir(current_dir)
        .arg(source)
        .spawn()
        .expect("error executing")
        .wait();
}
