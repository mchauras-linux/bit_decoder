use function_name::named;
use std::process::exit;

const MSR_BIT_PR: u64 = 49;
const MSR_BIT_SW: u64 = 5;

const MSR_PR: u64 = 1 << (63 - MSR_BIT_PR);

pub(crate) struct Msr {
    value: u64,
}

impl Msr {
    pub(crate) fn new(reg_value: &str) -> Self {
        let val = get_u64_from_str(reg_value);
        Msr { value: val }
    }

    pub(crate) fn print(&self) {
        iterate_bits(self.value);
    }
}

fn sf_bit(bit: u64) {
    match bit {
        0 => println!("32-bit mode."),
        1 => println!("64-bit mode."),
        _ => (),
    }
}

fn hv_bit(bit: u64, reg: u64) {
    match bit {
        0 => println!("Not in Hypervisor state."),
        1 => {
            if reg & MSR_PR == 0 {
                println!("Hypervisor state.")
            } else {
                println!("Not in Hypervisor state.")
            }
        }
        _ => (),
    }
}

fn sw_bit(bit: u64) {
    if bit == 1 {
        println!(
            "Bit {} should be 0. Otherwise the results are undefined",
            MSR_BIT_SW
        );
    }
}

#[named]
fn vec_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn vsx_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn s_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn ee_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn pr_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn fp_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn me_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn fe0_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn te_bit(bit: u64, msr: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn fe1_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}
#[named]
fn ir_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn dr_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn pmm_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

#[named]
fn ri_bit(bit: u64) {
    if bit == 1 {
        println!("{} is set.", function_name!());
    }
}

fn le_bit(bit: u64) {
    match bit {
        0 => println!("Big-Endian mode."),
        1 => println!("Little-Endian mode."),
        _ => (),
    }
}

fn iterate_bits(value: u64) {
    let mut unused_bits: Vec<u64> = vec![];
    for i in (0..64).rev() {
        let bit = (value >> (63 - i)) & 1;
        match i {
            0 => sf_bit(bit),
            3 => hv_bit(bit, value),
            5 => sw_bit(bit),
            38 => vec_bit(bit),
            40 => vsx_bit(bit),
            41 => s_bit(bit),
            48 => ee_bit(bit),
            49 => pr_bit(bit),
            50 => fp_bit(bit),
            51 => me_bit(bit),
            52 => fe0_bit(bit),
            53 => te_bit(bit, value),
            54 => (),
            55 => fe1_bit(bit),
            58 => ir_bit(bit),
            59 => dr_bit(bit),
            61 => pmm_bit(bit),
            62 => ri_bit(bit),
            63 => le_bit(bit),
            _ => {
                if bit == 1 {
                    unused_bits.push(i)
                }
            }
        }
    }
    if unused_bits.is_empty() {
        return;
    }
    for i in unused_bits {
        println!("Reserved bit {i} is set.")
    }
    println!("Refer HW Implementation details for more info on these")
}

fn hex_to_u64(hex_str: &str) -> Result<u64, std::num::ParseIntError> {
    let hex_str = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };
    u64::from_str_radix(hex_str, 16)
}

fn get_u64_from_str(val: &str) -> u64 {
    match hex_to_u64(val) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{}: Not a hex value {}", e, val);
            exit(-1);
        }
    }
}
