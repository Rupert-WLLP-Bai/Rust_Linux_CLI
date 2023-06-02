// use confique crate to implement a config file

use confique::Config;
use serde::{Deserialize, Serialize};

// define a struct to store config data
#[derive(Debug, Serialize, Deserialize,Config)]
struct Conf {
    // app name
    #[config(default = "confique_demo")]
    app_name: String,
    // Db config
    #[config(nested)]
    db: DbConf,
    // Server config
    #[config(nested)]
    server: ServerConf,
    // Log config
    #[config(nested)]
    log: LogConf,
}

// Db config
#[derive(Debug, Serialize, Deserialize, Config)]
struct DbConf {
    // Db name
    #[config(default = "confique_demo")]
    db_name: String,
    // Db type
    #[config(default = "postgres")]
    db_type: String,
    // Db host
    #[config(default = "localhost")]
    db_host: String,
    // Db port
    #[config(default = 5432)]
    db_port: u16,
    // Db user
    #[config(default = "postgres")]
    db_user: String,
    // Db password
    #[config(default = "postgres")]
    db_password: String,
    // Db pool size
    #[config(default = 10)]
    db_pool_size: u16,
}

// Server config
#[derive(Debug, Serialize, Deserialize, Config)]
struct ServerConf {
    // Server host
    #[config(default = "localhost")]
    server_host: String,
    // Server port
    #[config(default = 8080)]
    server_port: u16,
}

// Log config
#[derive(Debug, Serialize, Deserialize, Config)]
struct LogConf {
    // Log level
    #[config(default = "info")]
    log_level: String,
    // Log path
    #[config(default = "log")]
    log_path: String,
}

fn main(){
    let conf = Conf::builder().env().load().unwrap();
    println!("{:#?}", conf);
}

#[test]
fn test_config() {
    let conf = Conf::builder().env().load().unwrap();
    println!("{:#?}", conf);
    assert_eq!(conf.app_name, "confique_demo");
    assert_eq!(conf.db.db_name, "confique_demo");
    assert_eq!(conf.db.db_type, "postgres");
    assert_eq!(conf.db.db_host, "localhost");
    assert_eq!(conf.db.db_port, 5432);
    assert_eq!(conf.db.db_user, "postgres");
    assert_eq!(conf.db.db_password, "postgres");
    assert_eq!(conf.db.db_pool_size, 10);
    assert_eq!(conf.server.server_host, "localhost");
    assert_eq!(conf.server.server_port, 8080);
    assert_eq!(conf.log.log_level, "info");
    assert_eq!(conf.log.log_path, "log");
}