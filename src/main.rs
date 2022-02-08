mod config {
    const EXAMPLE_CONF: &'static str = r#"{
    "http_port": 7890,
    "socks5_port": 7891,
    "redir_port": 7892,
    "mixed_port": 7893,
    "allow_lan": true,
    "external_controller": "127.0.0.1:9090",
    "subscribe_url": "",
    "is_subscribe_banned": false,
    "custom_rules": [],
    "mmdb_file_url": "http://www.ideame.top/mmdb/Country.mmdb",
    "mmdb_version_url": "http://www.ideame.top/mmdb/version",
    "periodically_update": false
}"#;

    type Port = u16;

    use std::io::Write;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        http_port: Port,
        socks5_port: Port,
        redir_port: Port,
        mixed_port: Port,
        allow_lan: bool,
        external_controller: String,
        subscribe_url: String,
        is_subscribe_banned: bool,
        custom_rules: Vec<String>,
        mmdb_file_url: String,
        mmdb_version_url: String,
        periodically_update: bool,
    }

    use anyhow::Result;

    impl Config {
        pub fn from_path(conf_file_path: &std::path::Path) -> Result<Self, anyhow::Error> {
            if !conf_file_path.is_file() {
                let mut file = std::fs::File::create(conf_file_path).unwrap();
                file.write(EXAMPLE_CONF.as_ref()).unwrap();
            }
            let raw = std::fs::read(conf_file_path)?;
            let raw = std::str::from_utf8(&raw)?;
            let config: Self = serde_json::from_str(raw)?;
            Ok(config)
        }
    }
}

#[test]
fn test_load_config() {
    let config = config::Config::from_path(std::path::Path::new("C:\\Users\\fffzlfk\\.config\\clash\\clashup.json")).unwrap();
    println!("{:?}", config);
}

use std::path::Path;

#[derive(Debug)]
struct ClashUp<'a> {
    clash_conf_path: &'a Path,
    clash_conf_old_path: &'a Path,
    cache_file_path: &'a Path,
    mmdb_version_file_path: &'a Path,
    mmdb_file_path: &'a Path,
    config: config::Config,
}

impl ClashUp<'static> {
    fn new() -> Self {
        Self {
            clash_conf_path: Path::new("~/.config/clash/config.yaml"),
            clash_conf_old_path: Path::new("~/.config/clash/config.yaml.old"),
            cache_file_path: Path::new("~/.cache/clashup"),
            mmdb_version_file_path: Path::new("~/.cache/clashup-mmdb"),
            mmdb_file_path: Path::new("~/.cache/clashup-mmdb"),
            config: config::Config::from_path(Path::new("C:\\Users\\fffzlfk\\.config\\clash\\clashup.json")).unwrap(),
        }
    }
}

#[test]
fn test_new_clash_up() {
    let clash_up = ClashUp::new();
    println!("{:?}", clash_up);
}

fn main() {
    println!("Hello, world!");
}
