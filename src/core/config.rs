use config::Config;
use lazy_static::lazy_static;
use log::info;

lazy_static! {
    pub static ref CONFIG: Config = {
        info!("Config initialized");
        let config =  Config::builder()
                                .add_source(config::File::with_name("src/config.yaml"))
                                .build()
                                .unwrap();
        config
    };
}
