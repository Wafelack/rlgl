use nix::unistd::{fork, ForkResult};
use std::{time::Duration, fs, process::{Command, exit}, io::{stdin, BufRead}, os::unix::fs::MetadataExt};
use clap::{App, Arg};
use std::thread;

#[derive(Debug)]
pub struct Error(pub String);

macro_rules! error {
    ($($arg:tt)*) => {
        Err(Error(format_args!($($arg)*).to_string()))
    }
}
pub type Result<T> = std::result::Result<T, Error>;

fn get_edit_time(file: String) -> Result<i64>{
    match fs::metadata(&file) {
        Ok(m) => Ok(m.mtime()),
        Err(e) => error!("Failed to query file metadata: {}: {}.", file, e),
    }
}

fn try_main() -> Result<()>{
    let matches = App::new("rlgl")
                    .about("Play red light, green light with files.")
                    .version(env!("CARGO_PKG_VERSION"))
                    .arg(Arg::with_name("command")
                         .required(true)
                         .index(1)
                         .multiple(true)
                         .min_values(1))
                    .get_matches();
    let mut raw_command = matches.values_of("command").unwrap();
    let command = raw_command.next().unwrap();
    let args = raw_command.collect::<Vec<&str>>();

    let files = stdin().lock().lines().map(|l| match l {
        Ok(l) => Ok(l),
        Err(_) => error!("Failed to read file name from standard input."),
    }).collect::<Result<Vec<_>>>()?.into_iter().filter(|f| !f.is_empty()).collect::<Vec<_>>();
    let mut edit_dates = files.iter().map(|f| {
        get_edit_time(f.to_string())
    }).collect::<Result<Vec<i64>>>()?;

    match fork() {
        Ok(ForkResult::Parent { .. }) => Ok(()),
        Ok(ForkResult::Child) => {
            loop { 
                thread::sleep(Duration::from_secs(1));
                let new_edits = files.iter().map(|f| {
                    get_edit_time(f.to_string())
                }).collect::<Result<Vec<i64>>>()?;

                for (idx, new) in new_edits.iter().enumerate() {
                    let prev = edit_dates[idx];
                    let fname = &files[idx];

                    if *new > prev {
                        println!("\x1b[0;32m*\x1b[0m A file has changed: {}.", fname);
                        let status = match Command::new(command)
                                .args(&args)
                                .status() {
                                    Ok(s) => s,
                                    Err(e) => return error!("Failed to run command: {}.", e),
                                };
                        if !status.success() {
                            println!("Command failed, exit code: {}.", status.code().unwrap_or(1));
                            // Check if we need to exit the program with the strict flag.
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
            eprintln!("rlgl: {}", e.0);
            exit(1);
        }
    }
}
