pub mod base64;
pub mod csv;
pub mod genpass;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::CsvOpts;
pub use self::csv::OutputFormat;
pub use self::genpass::GenPassOpts;
pub use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        let filename = "Cargo.toml";
        assert_eq!(verify_input_file(filename), Ok(filename.into()));
        let filename = "not_exist";
        assert_eq!(verify_input_file(filename), Err("File does not exist"));
        let filename = "-";
        assert_eq!(verify_input_file(filename), Ok(filename.into()));
        let filename = "*";
        assert_eq!(verify_input_file(filename), Err("File does not exist"));
    }
}
