mod log_macros;

use clap::Parser;
use core::fmt::Arguments;
use std::error::Error;
use std::path::PathBuf;

pub trait RustCliQuickStartLog {
    fn output(self: &Self, args: Arguments);
    fn warning(self: &Self, args: Arguments);
    fn error(self: &Self, args: Arguments);
}

pub struct RustCliQuickStartTool<'a> {
    log: &'a dyn RustCliQuickStartLog,
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Specify the JSON data file
    #[clap(value_name = "INPUT_FILE")]
    input_file: PathBuf,

    #[clap(long = "output", short, value_name = "OUTPUT_FILE")]
    output_file: PathBuf,
}
impl<'a> RustCliQuickStartTool<'a> {
    pub fn new(log: &'a dyn RustCliQuickStartLog) -> RustCliQuickStartTool {
        RustCliQuickStartTool { log }
    }

    pub fn run(
        self: &mut Self,
        args: impl IntoIterator<Item = std::ffi::OsString>,
    ) -> Result<(), Box<dyn Error>> {
        let _cli = match Cli::try_parse_from(args) {
            Ok(m) => m,
            Err(err) => {
                output!(self.log, "{}", err.to_string());
                return Ok(());
            }
        };

        Ok(())
    }
}
