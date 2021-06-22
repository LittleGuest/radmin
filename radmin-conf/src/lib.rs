#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use toml::Value;

const SETTING_CONFIG_FILE_PATH: &str = "./setting.toml";
const ACTIVE_PROFILE_KEY: &str = "profile";

// #[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
// pub enum Environment {
//     /// 开发环境
//     Dev,
//     /// 生产环境
//     Prod,
// }

#[derive(Debug)]
pub struct Config {
    pub active_profile: String,
    pub configs: HashMap<String, AppConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub name: Option<String>,
    pub port: u16,
    pub mysql: Option<MysqlConfig>,
    pub redis: Option<RedisConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MysqlConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: Option<u8>,
}

lazy_static! {
    static ref APP_CONFIG: Config = {
        let mut config_file =
            File::open(SETTING_CONFIG_FILE_PATH).expect("open setting config file failed");
        let mut config_content = String::new();
        config_file
            .read_to_string(&mut config_content)
            .expect("open setting config file failed");

        let tv = toml::from_str::<toml::Value>(&config_content)
            .expect("toml parse setting config file failed");

        let active_profile =
            std::env::var(ACTIVE_PROFILE_KEY).unwrap_or_else(|_| "dev".to_string());
        println!("active_profile = {:?}", active_profile);

        match tv {
            Value::Table(v) => {
                let json_string = serde_json::to_string(&v).unwrap();
                let configs: HashMap<String, AppConfig> =
                    serde_json::from_str(&json_string).unwrap();
                println!("{:#?}", configs);

                Config {
                    active_profile,
                    configs,
                }
            }
            _ => {
                panic!("the setting config file is not we are excepting")
            }
        }
    };
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active_profile: "dev".to_string(),
            configs: {
                let mut map = HashMap::new();
                let app_config = AppConfig {
                    name: Some("radmin".to_string()),
                    port: 10241,
                    mysql: Some(MysqlConfig {
                        url: "127.0.0.1".to_string(),
                        username: "root".to_string(),
                        password: "root".to_string(),
                    }),
                    redis: Some(RedisConfig {
                        host: "127.0.0.1".to_string(),
                        port: 6379,
                        password: None,
                        db: None,
                    }),
                };
                map.insert("dev".to_string(), app_config);
                map
            },
        }
    }
}

impl Config {
    pub fn get_app_config() -> AppConfig {
        let app_config = APP_CONFIG
            .configs
            .get(APP_CONFIG.active_profile.as_str())
            .expect("get app config failed");

        // FIXME to be optimized
        AppConfig {
            name: app_config.name.clone(),
            port: app_config.port,
            mysql: app_config.mysql.clone(),
            redis: app_config.redis.clone(),
        }
    }
    pub fn get_mysql_config() -> MysqlConfig {
        Self::get_app_config()
            .mysql
            .expect("fetch mysql config failed")
    }

    pub fn get_redis_config() -> RedisConfig {
        Self::get_app_config()
            .redis
            .expect("fetch redis config failed")
    }
}

#[cfg(test)]
#[test]
fn test_conf() {
    let mysql_config = Config::get_mysql_config();
    println!("{:?}", mysql_config);

    let redis_config = Config::get_redis_config();
    println!("{:?}", redis_config);
}
