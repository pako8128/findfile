use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use structopt::StructOpt;

use exitfailure::ExitFailure;

use log::{error, info};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    human_panic::setup_panic!();
    env_logger::init();

    info!("Reading commandline argument");
    let args = Cli::from_args();

    if !args.path.is_dir() {
        error!("{:?} is not a directory", args.path);
        process::exit(exitcode::USAGE);
    }

    findfile::find(
        &args.pattern,
        &args.path,
        Rc::new(RefCell::new(std::io::stdout())),
    )?;

    process::exit(exitcode::OK);
}
