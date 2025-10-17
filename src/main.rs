mod config;

use config::Config;

fn main() {
    let path = "config.yaml";

    let config = Config::from_file(path).unwrap();

    println!("The address is {}", config.server);

}


