use structopt::StructOpt;
use nix::unistd::{fork, ForkResult};
use std::thread;
use std::{
    fs,
    io::{stdin, BufRead},
    os::unix::fs::MetadataExt,
    process::{exit, Command},
    time::{Duration, Instant},
};

#[derive(StructOpt)]
#[structopt(name = "rlgl", about = "Play red light, green light with files.", help_message = "Print help information.", version_message = "Print version information.")]
struct Rlgl {
    #[structopt(short, long, help = "Display information about what is going on.")]
    verbose: bool,
    #[structopt(short, long, help = "Exit on the first failed command.")]
    strict: bool,
    #[structopt(short, long, default_value = "1.0", help = "The delay in seconds between 2 checks.")]
    delay: f32,
    #[structopt(short, long, default_value = "-1", help = "The time to live (in seconds) before killing the process. -1 means infinity.")]
    ttl: i32,
    #[structopt(short, long, help = "Do not display command output.")]
    quiet: bool,
    #[structopt(value_name = "COMMAND", help = "The command to run when a file changes.")]
    command: String,
    #[structopt(value_name = "ARGS", multiple = true, help = "The arguments to give to the COMMAND.")]
    args: Vec<String>,
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
    let args = Rlgl::from_args();

    let ttl = if args.ttl < 0 { None } else { Some(args.ttl) };

    let files = stdin()
        .lock()
        .lines()
        .map(|l| match l {
            Ok(l) => Ok(l.trim().to_string()),
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
        Ok(ForkResult::Parent { child }) => {
            println!("{} rlgl: Child PID {}.", GREEN_STAR, child);
            Ok(())
        },
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
                thread::sleep(Duration::from_secs_f32(args.delay));
                let new_edits = files
                    .iter()
                    .map(|f| get_edit_time(f.to_string()))
                    .collect::<Result<Vec<i64>>>()?;

                for (idx, new) in new_edits.iter().enumerate() {
                    let prev = edit_dates[idx];
                    let fname = &files[idx];

                    if *new > prev {
                        if args.verbose {
                            println!("{} rlgl: `{}` has changed.", GREEN_STAR, fname);
                        }
                        let (status, stdout) = match Command::new(&args.command).args(&args.args).output() {
                            Ok(s) => (s.status, s.stdout),
                            Err(e) => {
                                if args.verbose {
                                    eprintln!(
                                        "{} rlgl: `{}`: {}.",
                                        RED_STAR,
                                        format!("{} {}", args.command, args.args.join(" ")),
                                        e
                                        );
                                }
                                if args.strict {
                                    return error!(
                                        "Failed to execute `{}`: {}.",
                                        format!("{} {}", args.command, args.args.join(" ")),
                                        e
                                        );
                                } else {
                                    continue;
                                }
                            }
                        };
                        if !args.quiet {
                            println!("{}", String::from_utf8_lossy(&stdout).trim());
                        }
                        if !status.success() {
                            if args.verbose {
                                eprintln!(
                                    "{} rlgl: `{}` failed, exit code: {}.",
                                    RED_STAR,
                                    format!("{} {}", args.command, args.args.join(" ")),
                                    status.code().unwrap_or(1)
                                    );
                            }
                            if args.strict {
                                return error!(
                                    "`{}` execution failed. Aborting due to `--strict`.",
                                    format!("{} {}", args.command, args.args.join(" "))
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
