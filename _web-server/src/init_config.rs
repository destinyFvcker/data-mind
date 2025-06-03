use clap::{command, Parser};
use config::{Environment, File};
use serde::Deserialize;

/// data-mind 网页服务器
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 要读取的文件路径
    #[arg(short, long)]
    config_path: String,
}

#[derive(Debug, Deserialize)]
pub struct InitConfig {
    pub server: ServerConfig,
    pub mysql: MysqlConfig,
    pub coze: CozeConfig,
    pub clickhouse: ClickhouseConfig,
    pub jwt_secret_key: String,
    pub github: GithubConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub fe: String,
    pub logdir: String,
    pub deploy_path: String,
}

/// Coze平台智能体的相关配置
#[derive(Debug, Deserialize)]
pub struct CozeConfig {
    /// Coze应用id
    pub id: String,
    /// Coze OAuth应用的公钥指纹
    pub kid: String,
    /// Coze OAuth应用的私钥签名
    pub signingkey: String,
}

#[derive(Debug, Deserialize)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct ClickhouseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubConfig {
    /// 注册时从 GitHub 收到的客户端 ID。
    pub client_id: String,
    /// github 签发的 client_secret
    pub secret: String,
}

impl InitConfig {
    pub fn new() -> anyhow::Result<Self> {
        let args = Args::parse();
        let config = config::Config::builder()
            .add_source(File::with_name(&args.config_path))
            .add_source(
                Environment::with_prefix("clickhouse")
                    .keep_prefix(true)
                    .separator("_"),
            )
            .add_source(
                Environment::with_prefix("mysql")
                    .keep_prefix(true)
                    .separator("_"),
            )
            .add_source(
                Environment::with_prefix("server")
                    .keep_prefix(true)
                    .separator("_"),
            )
            .add_source(
                Environment::with_prefix("coze")
                    .keep_prefix(true)
                    .separator("_"),
            )
            .add_source(Environment::with_prefix("jwt").keep_prefix(true))
            .add_source(
                Environment::with_prefix("github")
                    .keep_prefix(true)
                    .separator("_"),
            )
            .build()?
            .try_deserialize::<Self>()?;

        Ok(config)
    }
}
