use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use toml::from_str;

lazy_static! {
    // CFG variables defined in cfg.toml file
    pub static ref CFG: HashMap<&'static str, String> = {
        let cfg_str = include_str!("../../cfg.toml");
        let config: Config = from_str(cfg_str).unwrap();

        let mut map = HashMap::new();

        map.insert("site.title", config.site.title);

        map.insert("gql.addr", config.gql.addr);
        map.insert("gql.port", config.gql.port.to_string());
        map.insert("gql.path",config.gql.path);

        map
    };
}

#[derive(Deserialize)]
struct Config {
    site: Site,
    gql: Gql,
}

#[derive(Deserialize)]
struct Site {
    title: String,
}

#[derive(Deserialize)]
struct Gql {
    addr: String,
    port: u16,
    path: String,
}
