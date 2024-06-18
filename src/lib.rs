use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use serde_json::Result as SResult;
pub mod calc;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub fixed_charge: f32,
    pub fppca_charge: f32,
    pub tax: f32,
    pub energy_unit: f32,
    pub energy_rate: f32,
    pub dinesh_cr: f32,
    pub sachin_cr: f32,
    pub sankit_cr: f32
}

#[derive(Deserialize, Debug)]
pub struct Readings {
    pub dinesh_reading: f32,
    pub sachin_reading: f32,
    pub sankit_reading: f32
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Bill {
    pub dinesh_bill: f32,
    pub sachin_bill: f32,
    pub sankit_bill: f32,
    pub kaniska_bill: f32
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("More arguments passed than necessary!");
        }

        let file_path = args[1].clone();
        let output = match read_config_from_file(file_path) {
            Ok(config) => {
                print_config(&config);
                Ok(config)
            }
            Err(e) => Err("Error reading json file"),
        };

        let current_path = "reading.json".to_string();
        let current = match read_config_from_file(current_path) {
            Ok(config) => {
                print_reading(&config);
                Ok(config)
            }
            Err(e) => Err("Error reading json file"),
        };

        output
    }
}

fn read_config_from_file<T: DeserializeOwned>(file_path: String) -> SResult<T> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let content = serde_json::from_reader(reader).unwrap();
    Ok(content)
}

fn print_config(config: &Config) {
    println!("fixed_charge: {}", config.fixed_charge);
    println!("fppca_charge: {}", config.fppca_charge);
    println!("tax: {}", config.tax);
    println!("energy_unit: {}", config.energy_unit);
    println!("energy_rate: {}", config.energy_rate);
}

fn print_reading(config: &Readings) {
    println!("dinesh_reading: {}", config.dinesh_reading);
    println!("sachin_reading: {}", config.sachin_reading);
    println!("sankit_reading: {}", config.sankit_reading);
}
