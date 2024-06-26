use crate::Readings;
use crate::Config;
use crate::Bill;

fn round_to_decimals(num: f32, decimals: u32) -> f32 {
    let factor = 10_f32.powi(decimals as i32);
    (num * factor).round() / factor
}

pub fn calculate(config: &Config, readings: &Readings) -> Result<Bill, &'static str> {
    let mut non_energy_changes: f32 = (config.fixed_charge + config.fppca_charge + config.tax) / 4.0;
    
    let dinesh_usage = config.dinesh_cr - readings.dinesh_reading;
    let sachin_usage = config.sachin_cr - readings.sachin_reading;
    let sankit_usage = config.sankit_cr - readings.sankit_reading;
    let current_ac_unit: f32 = dinesh_usage + sachin_usage + sankit_usage;

    let non_ac_unit = config.energy_unit - current_ac_unit;
    non_energy_changes += (non_ac_unit * config.energy_rate) / 4.0;

    let result = Bill {
        dinesh_bill: round_to_decimals(non_energy_changes + dinesh_usage * config.energy_rate, 1),
        sachin_bill: round_to_decimals(non_energy_changes + sachin_usage * config.energy_rate, 1),
        sankit_bill: round_to_decimals(non_energy_changes + sankit_usage * config.energy_rate, 1),
        kaniska_bill: round_to_decimals(non_energy_changes, 2)
    };

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate() {
        let config = Config {
            fixed_charge: 480.0,
            fppca_charge: 79.12,
            tax: 525.16,
            energy_unit: 989.0,
            energy_rate: 5.9,
            dinesh_cr: 15.0,
            sachin_cr: 50.0,
            sankit_cr: 87.0
        };

        let reading = Readings {
            dinesh_reading: 10.0,
            sachin_reading: 20.0,
            sankit_reading: 30.0
        };

        let result = Bill {
            dinesh_bill: 1623.65,
            sachin_bill: 1771.15,
            sankit_bill: 1930.45,
            kaniska_bill: 1594.15
        };

        assert_eq!(result, calculate(&config, &reading).expect("value returns False"));
    }
}