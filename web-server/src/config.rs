use std::sync::LazyLock;

use clap::{command, Parser};
use config::File;
use serde::Deserialize;
use snafu::{ResultExt, Whatever};

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new().unwrap());

/// data-mind 网页服务器
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 要读取的文件路径
    #[arg(short, long)]
    config_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u32,
    pub fe: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
}

impl Config {
    fn new() -> Result<Self, Whatever> {
        let args = Args::parse();
        let s = config::Config::builder()
            .add_source(File::with_name(&args.config_path))
            .build()
            .with_whatever_context(|err| format!("{:#?}", err))?;

        Ok(s.try_deserialize()
            .with_whatever_context(|err| format!("{:#?}", err))?)
    }
}
