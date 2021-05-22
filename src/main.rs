use clap::{App, Arg};
use nix::unistd::{fork, ForkResult};
use std::thread;
use std::{
    fs,
    io::{stdin, BufRead},
    os::unix::fs::MetadataExt,
    process::{exit, Command},
    time::{Duration, Instant},
};

macro_rules! gen_app {
    () => {
App::new("rlgl")
        .about("Play red light, green light with files.")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .help_message("Print help information.")
        .version_message("Print version information.")
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("Display information about what is going on."))
        .arg(Arg::with_name("strict")
             .short("s")
             .long("strict")
             .help("Exit on the first failed command."))
        .arg(Arg::with_name("delay")
             .short("d")
             .long("delay")
             .takes_value(true)
             .help("The delay in seconds between 2 checks. Defaults to 1.0."))
        .arg(Arg::with_name("ttl")
             .short("t")
             .long("ttl")
             .takes_value(true)
             .help("The time to live in seconds before killing the child process. -1 means infinite. Defaults to -1."))
        .arg(Arg::with_name("quiet")
             .short("q")
             .long("quiet")
             .help("Redirect command output to /dev/null."))
        .arg(Arg::with_name("command")
             .required(true)
             .index(1)
             .value_name("COMMAND")
             .help("The command to run."))
        .arg(Arg::with_name("arguments")
             .index(2)
             .multiple(true)
             .value_name("ARGUMENTS")
             .help("The arguments to passe to COMMAND."))

    }
}
const GREEN_STAR: &str = "\x1b[0;32m*\x1b[0m";
const RED_STAR: &str = "\x1b[0;31m*\x1b[0m";

#[derive(Debug)]
pub struct Error(pub String);

macro_rules! error {
    ($($arg:tt)*) => {
        Err(Error(format_args!($($arg)*).to_string()))
    }
}
pub type Result<T> = std::result::Result<T, Error>;

fn get_edit_time(file: String) -> Result<i64> {
    match fs::metadata(&file) {
        Ok(m) => Ok(m.mtime()),
        Err(e) => error!("Failed to query file metadata: {}: {}.", file, e),
    }
}

fn try_main() -> Result<()> {
    let matches = gen_app!().get_matches();

    let command = matches.value_of("command").unwrap();
    let args = matches.values_of("arguments").and_then(|args| Some(args.collect::<Vec<&str>>())).unwrap_or(vec![]);
    let verbose = matches.is_present("verbose");
    let quiet = matches.is_present("quiet");
    let strict = matches.is_present("strict");
    let ttl = match matches.value_of("ttl") {
        Some(v) => match v.parse::<i32>() {
            Ok(v) => {
                if v < 0 {
                    None
                } else {
                    Some(v)
                }
            }
            Err(_) => None,
        },
        None => None,
    };
    let delay = match matches.value_of("delay") {
        Some(v) => v.parse::<f32>().unwrap_or(1.0),
        None => 1.0,
    };

    let files = stdin()
        .lock()
        .lines()
        .map(|l| match l {
            Ok(l) => Ok(l),
            Err(_) => error!("Failed to read file name from standard input."),
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|f| !f.is_empty())
        .collect::<Vec<_>>();
    let mut edit_dates = files
        .iter()
        .map(|f| get_edit_time(f.to_string()))
        .collect::<Result<Vec<i64>>>()?;

    match fork() {
        Ok(ForkResult::Parent { .. }) => Ok(()),
        Ok(ForkResult::Child) => {
            let start = Instant::now();
            loop {
                match ttl {
                    Some(ttl) => {
                        if start.elapsed().as_secs() > ttl as u64 {
                            return Ok(());
                        }
                    }
                    None => {}
                }
                thread::sleep(Duration::from_secs_f32(delay));
                let new_edits = files
                    .iter()
                    .map(|f| get_edit_time(f.to_string()))
                    .collect::<Result<Vec<i64>>>()?;

                for (idx, new) in new_edits.iter().enumerate() {
                    let prev = edit_dates[idx];
                    let fname = &files[idx];

                    if *new > prev {
                        if verbose {
                            println!("{} rlgl: `{}` has changed.", GREEN_STAR, fname);
                        }
                        let (status, stdout) = match Command::new(command).args(&args).output() {
                            Ok(s) => (s.status, s.stdout),
                            Err(e) => {
                                if verbose {
                                    eprintln!(
                                        "{} rlgl: `{}`: {}.",
                                        RED_STAR,
                                        format!("{} {}", command, args.join(" ")),
                                        e
                                    );
                                }
                                if strict {
                                    return error!(
                                        "Failed to execute `{}`: {}.",
                                        format!("{} {}", command, args.join(" ")),
                                        e
                                    );
                                } else {
                                    continue;
                                }
                            }
                        };
                        if !quiet {
                            println!("{}", String::from_utf8_lossy(&stdout).trim());
                        }
                        if !status.success() {
                            if verbose {
                                eprintln!(
                                    "{} rlgl: `{}` failed, exit code: {}.",
                                    RED_STAR,
                                    format!("{} {}", command, args.join(" ")),
                                    status.code().unwrap_or(1)
                                );
                            }
                            if strict {
                                return error!(
                                    "`{}` execution failed. Aborting due to `--strict`.",
                                    format!("{} {}", command, args.join(" "))
                                );
                            }
                        }
                        break;
                    }
                }
                edit_dates = new_edits;
            }
        }
        Err(_) => {
            error!("Failed to fork process.")
        }
    }
}

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{} rlgl: {}", RED_STAR, e.0);
            exit(1);
        }
    }
}
