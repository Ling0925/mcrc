pub mod launcher {
    use self::config_parser::Config;

    mod config_parser;

    pub fn launcher() -> &'static str {
        let mut config = config_parser::Config::init();
        // let mut config = Config::new();
        // config.update_config();
        // config.parser_to_file().expect("Connot save config.");
        println!("{:?}", config);
        "Launcher...."
    }
}
