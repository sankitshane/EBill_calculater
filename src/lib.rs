use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use serde_json::Result as SResult;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub fixed_charge: f32,
    pub fppca_charge: f32,
    pub tax: f32,
    pub energy_unit: f32,
    pub energy_rate: f32
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("More arguments passed than necessary!");
        }

        let file_path = args[1].clone();
        let output = match read_config_from_file(file_path) {
            Ok(config) => {
                my_function(&config);
                Ok(config)
            }
            Err(e) => Err("Error reading json file"),
        };
        output
    }
}

fn read_config_from_file(file_path: String) -> SResult<Config> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader).unwrap();
    Ok(config)
}

fn my_function(config: &Config) {
    println!("fixed_charge: {}", config.fixed_charge);
    println!("fppca_charge: {}", config.fppca_charge);
    println!("tax: {}", config.tax);
    println!("energy_unit: {}", config.energy_unit);
    println!("energy_rate: {}", config.energy_rate);
}
