use mysql::OptsBuilder;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,

    // specifying DBs to include from the dump
    pub db_include: Vec<String>,

    // specifying DBs to exclude from the dump
    pub db_exclude: Vec<String>,

    // choose Snaphost folder
    pub db_folder: String,

    // Choose algorithm compression
    pub compression: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let db_include: Vec<String> = env::var("DB_EXPORTS")?
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let db_exclude: Vec<String> = env::var("DB_EXCLUDE")?
            .split(',')
            .map(|s| s.to_string())
            .collect();

        Ok(Self {
            db_host: env::var("DB_HOST")?,
            db_port: env::var("DB_PORT")?
                .parse::<u16>()
                .map_err(|_| env::VarError::NotPresent)?,
            db_username: env::var("DB_USERNAME")?,
            db_password: env::var("DB_PASSWORD")?,
            db_folder: env::var("DB_FOLDER")?,
            db_include,
            db_exclude,
            compression: env::var("COMPRESSION")?,
        })
    }

    #[allow(dead_code)]
    pub fn mysql_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.db_username, self.db_password, self.db_host, self.db_port
        )
    }

    pub fn mysql_opts(&self) -> OptsBuilder {
        let builder = OptsBuilder::new()
            .ip_or_hostname(Some(&self.db_host))
            .tcp_port(self.db_port)
            .user(Some(&self.db_username))
            .pass(Some(&self.db_password));
        builder
    }
}
