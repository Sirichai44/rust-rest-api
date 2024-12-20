use std::sync::Arc;

use config::{Config, ConfigError, File};
use urlencoding::encode;

pub struct Server {
    pub port: i64,
}

pub struct Database {
    pub host: String,
    pub port: i64,
    pub username: String,
    pub password: String,
}

pub struct Setting {
    pub server: Server,
    pub database: Database,
}

impl Setting {
    pub fn new() -> Result<Arc<Self>, ConfigError> {
        let setting = Config::builder()
            .add_source(File::with_name("Setting"))
            .build()
            .unwrap();

        Ok(Arc::new(Self {
            server: Server {
                port: setting.get_int("server.port")?,
            },
            database: Database {
                host: setting.get_string("database.host")?,
                port: setting.get_int("database.port")?,
                username: setting.get_string("database.username")?,
                password: setting.get_string("database.password")?,
            },
        }))
    }
}

impl Database {
    pub fn url_getting(&self) -> String {
        let enc_username = encode(&self.username);
        let enc_password = encode(&self.password);

        println!(
            "mongodb://{}:{}@{}:{}",
            enc_username, enc_password, self.host, self.port
        );
        format!(
            "mongodb://{}:{}@{}:{}",
            enc_username, enc_password, self.host, self.port
        )
    }
}
