use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 03!");

    let input: String = fs::read_to_string("./src/day03/input.txt")?;

    let banks: Vec<Bank> = parse_banks(&input)?;
    let max_total_voltage_p1 = get_max_total_voltage(&banks, 2);
    assert_eq!(max_total_voltage_p1, 17554);
    println!("Part 1: {:?}", &max_total_voltage_p1);
    let max_total_voltage_p2 = get_max_total_voltage(&banks, 12);
    assert_eq!(max_total_voltage_p2, 175053592950232);
    println!("Part 2: {:?}", &max_total_voltage_p2);
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
}

fn parse_banks(contents: &str) -> Result<Vec<Bank>, Box<dyn Error>> {
    let mut banks: Vec<Bank> = vec![];

    for line in contents.lines() {
        let voltages: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|x| x as u8)
            .collect();

        let bank = Bank { batts: voltages };
        banks.push(bank);
    }

    Ok(banks)
}

fn find_max_bank_voltage(banks: &[Bank], n_batteries_use: u64) -> Vec<Vec<Voltage>> {
    let mut bank_voltages: Vec<Vec<Voltage>> = vec![];
    for bank in banks {
        let n_batts = bank.batts.len();
        let mut start_idx: usize = 0;
        let mut stop_idx = n_batts - (n_batteries_use - 1) as usize;

        let mut voltages: Vec<Voltage> = vec![];
        for _ in 0..n_batteries_use {
            let max_idx: usize;
            let max_val: u8;
            if (start_idx != stop_idx) && (start_idx != n_batts - 1 as usize) {
                (max_idx, max_val) = bank.batts[..stop_idx]
                    .iter()
                    .enumerate()
                    .skip(start_idx)
                    .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
                    .map(|(i, &v)| (i, v))
                    .unwrap();
            } else {
                max_idx = start_idx;
                max_val = bank.batts[start_idx];
            }

            voltages.push(Voltage {
                val: max_val,
                pos: max_idx as u64,
            });
            start_idx = max_idx + 1;
            stop_idx += 1;
        }
        bank_voltages.push(voltages);
    }

    bank_voltages
}

fn get_bank_voltage(voltages: &[Voltage]) -> u64 {
    let mut mult_factor: u64 = 10u64.pow(voltages.len() as u32 - 1);
    let mut total_voltage: u64 = 0;

    for volt in voltages {
        total_voltage += mult_factor * volt.val as u64;
        mult_factor /= 10;
    }

    total_voltage
}

fn get_max_total_voltage(banks: &[Bank], n_batteries_use: u64) -> u64 {
    let total_voltage = find_max_bank_voltage(banks, n_batteries_use)
        .into_iter()
        .map(|x| get_bank_voltage(&x))
        .sum();

    total_voltage
}
