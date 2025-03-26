use std::sync::LazyLock;

use clap::Parser;
use config::File;
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
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Clickhouse {
    pub host: String,
    pub port: u32,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub clickhouse: Clickhouse,
}

impl Config {
    fn new() -> anyhow::Result<Config> {
        let args = Args::parse();
        let s = config::Config::builder()
            .add_source(File::with_name(&args.config_path))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}
