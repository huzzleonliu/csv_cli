use clap::Parser;

use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // 显示opts的信息
    // println!("{:?}", opts);
    // println!("===============================");

    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
