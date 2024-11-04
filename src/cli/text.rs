use super::verify_input_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    //如果不提供name的话，默认会将结构体名字转换为小写
    #[command(about = "Sign a text with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed text with a public/shared key")]
    Verity(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub key: String,
}
