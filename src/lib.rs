use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use serde_json::Result as SResult;
use std::error::Error;
use std::process;
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
        let config = match read_config_from_file(file_path) {
            Ok(output) => {
                Ok(output)
            }
            Err(_e) => Err("Error reading json file"),
        };

        config
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let readings_file_path = "reading.json".to_string();
    let readings = match read_config_from_file(readings_file_path) {
        Ok(output) => {
            Ok(output)
        }
        Err(_e) => Err("Error reading json file"),
    };
    
    let readings = readings.unwrap_or_else(|err| {
        eprintln!("Problem parsing readings file: {err}");
        process::exit(1);
    });

    let results = calc::calculate(&config, &readings);

    println!("{:#?}", results);

    Ok(())
}

fn read_config_from_file<T: DeserializeOwned>(file_path: String) -> SResult<T> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let content = serde_json::from_reader(reader).unwrap();
    Ok(content)
}
