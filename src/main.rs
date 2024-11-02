use clap::Parser;
use rcli::cli::base64::*;
use rcli::{process_csv, process_genpass, Opts, SubCommand};
use rcli::{process_decode, process_encode};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // 显示opts的信息
    // println!("{:?}", opts);
    // println!("===============================");

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }

    Ok(())
}
