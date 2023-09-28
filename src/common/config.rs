use crate::json_type::config::Config;
use std::fs::File;
use std::io::BufReader;


pub fn load_config() -> Config {
    let file = File::open("config.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = load_config();
}
