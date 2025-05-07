use std::sync::LazyLock;

use clap::Parser;
use config::{Environment, File};
use serde::Deserialize;

pub static INIT_CONFIG: LazyLock<InitConfig> = LazyLock::new(|| InitConfig::new().unwrap());

/// data-mind 网页服务器
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 要读取的文件路径
    #[arg(short, long)]
    config_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u32,
    pub logdir: String,
}

#[derive(Debug, Deserialize)]
pub struct ClickhouseConfig {
    pub host: String,
    pub port: u32,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub broker: String,
    pub topic: String,
    pub partition: i32,
}

#[derive(Debug, Deserialize)]
pub struct InitConfig {
    pub server: ServerConfig,
    pub clickhouse: ClickhouseConfig,
    pub kafka: KafkaConfig,
}

impl InitConfig {
    fn new() -> anyhow::Result<Self> {
        let args = Args::parse();
        let s = config::Config::builder()
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
            .build()?;

        Ok(s.try_deserialize()?)
    }
}
