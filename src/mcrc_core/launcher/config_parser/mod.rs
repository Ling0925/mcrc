mod user_config;

use std::{
    fs::File,
    io::{self, Read},
};

use serde::{Deserialize, Serialize, __private::de::Content};
use user_config::User;

// const CONFIG_FILE_PATH: &str = ".\\.minecraft\\config\\mcrc_config.json";
const CONFIG_FILE_PATH: &str = "mcrc_config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    game_ver: String,
    java_ver: String,
    pub(crate) user: User,
}

impl Config {
    pub fn parser_to_file(&self) -> io::Result<()> {
        let config_json = serde_json::to_string(&self).unwrap();
        std::fs::write(CONFIG_FILE_PATH, config_json)
    }
    pub fn update_config(&mut self) -> &Self {
        self.game_ver = String::from("1.18");
        self.java_ver = String::from("17");
        self.user.update();
        self
    }
}

impl Config {
    pub fn init() -> Config {
        println!("{}", CONFIG_FILE_PATH);
        let config_file = std::path::Path::new(CONFIG_FILE_PATH);
        if config_file.exists() {
            println!("Config file loading....");
            let mut content = String::new();
            let mut file = File::open(config_file).unwrap();
            file.read_to_string(&mut content)
                .expect("Cannot read config file.");
            Config::parser_for_file(content)
        } else {
            std::fs::File::create(CONFIG_FILE_PATH).expect("Cannot create file.");
            Config::new()
        }
    }
    pub fn new() -> Config {
        Config {
            game_ver: String::new(),
            java_ver: String::new(),
            user: User::new(),
        }
    }

    fn parser_for_file(config_json: String) -> Self {
        serde_json::from_str(&config_json).expect("Connot parser config file.")
    }
}
