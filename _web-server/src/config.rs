use std::sync::LazyLock;

use clap::{command, Parser};
use config::{Environment, File};
use serde::Deserialize;

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
    pub port: u16,
    pub fe: String,
    pub logdir: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
}

impl Config {
    fn new() -> anyhow::Result<Config> {
        let args = Args::parse();
        let config = config::Config::builder()
            .add_source(File::with_name(&args.config_path))
            .add_source(
                Environment::with_prefix("clickhouse")
                    .keep_prefix(true)
                    .separator("_"),
            )
            .add_source(
                Environment::with_prefix("server")
                    .keep_prefix(true)
                    .separator("_"),
            )
            .build()?
            .try_deserialize::<Config>()?;

        Ok(config)
    }
}
