use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 03!");

    let input: String = fs::read_to_string("./src/day03/input.txt")?;

    let (banks, max_voltage): (Vec<Bank>, u64) = parse_banks(&input)?;
    assert_eq!(max_voltage, 17554);
    println!("{:?}", &max_voltage);
    Ok(())
}

#[derive(Debug)]
struct Voltage {
    val: u8,
    pos: u64,
}

#[derive(Debug)]
struct Bank {
    batts: Vec<u8>,
    most_sig_dig: Voltage,
    least_sig_dig: Voltage,
}

impl Bank {
    fn combined_voltage(&self) -> u64 {
        let msd = self.most_sig_dig.val;
        let lsd = self.least_sig_dig.val;

        (msd as u64 * 10) + (lsd as u64)
    }
}

fn parse_banks(contents: &str) -> Result<(Vec<Bank>, u64), Box<dyn Error>> {
    let mut banks: Vec<Bank> = vec![];
    let mut max_voltage: u64 = 0;

    for line in contents.lines() {
        let voltages: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|x| x as u8)
            .collect();

        let (max_msd_idx, max_msd_val) = voltages[..voltages.len() - 1]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .map(|(i, &v)| (i, v))
            .unwrap();
        let (max_lsd_idx, max_lsd_val) = voltages
            .iter()
            .enumerate()
            .skip(max_msd_idx + 1)
            .max_by_key(|&(i, &v)| v)
            .map(|(i, &v)| (i, v))
            .unwrap();

        let bank = Bank {
            batts: voltages,
            most_sig_dig: Voltage {
                val: max_msd_val,
                pos: max_msd_idx as u64,
            },
            least_sig_dig: Voltage {
                val: max_lsd_val,
                pos: max_lsd_idx as u64,
            },
        };
        max_voltage += bank.combined_voltage();
        banks.push(bank);
    }

    Ok((banks, max_voltage))
}
