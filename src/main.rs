extern crate clap;
extern crate libc51902;

use std::char;
use std::cmp::Ordering;
use std::fs::{self, File};
use std::io::{self, Write};
use std::str::FromStr;

use clap::{Arg, App};
use libc51902::Opcode;

fn parse_machine_code(infile: &str, outfile: Option<&str>)
    -> Result<String, io::Error> {
    let infile_content: String = fs::read_to_string(&infile)?;
    let asm_vec: Vec<String> = infile_content.lines()
        .filter_map(|x| {
            if !x.is_empty() {
                Some(String::from(x))
            } else {
                None
            }
        })
        .collect();

    let mut bin_vec: Vec<String> = gen_bin_instr(&asm_vec);

    match bin_vec.len().cmp(&1024) {
        Ordering::Less => {
            while bin_vec.len() < 1024 {
                bin_vec.push(String::from("0".repeat(16)));
            }
        },
        Ordering::Greater => {
            panic!("Too many instructions!");
        },
        _ => ()
    }
    add_underscore_padding(&mut bin_vec);

    if let Some(out) = outfile {
        let mut of = File::create(out)?;
        writeln!(of, "{}", bin_vec.join("\n"))?;
    }

    Ok(infile_content)
}

fn gen_bin_instr(asm_vec: &Vec<String>) -> Vec<String> {
    let word_vec: Vec<Vec<String>> = asm_vec.iter()
        .map(|x|
            x.split_terminator(" ").map(|x| String::from(x))
                .collect()
        ).collect();

    // println!("{:?}", word_vec);

    let mut bin_vec: Vec<String> = Vec::new();

    for line in word_vec {
        let mut bin_instr = String::from("");
        let mut word = line.iter();
        let mnemonic_opt = Opcode::from_str(capitalize_str(word.next().unwrap())
            .as_str());

        // Ensures that the mnemonic opcode exists
        if let Ok(mnemonic) = mnemonic_opt {
            let d = mnemonic.to_binary().iter()
            .map(|&c| char::from_digit(c.into(), 10).unwrap())
            .collect::<String>();

            bin_instr.push_str(&d);

            match mnemonic {
                // nop
                Opcode::Nop => {
                    bin_instr.push_str("0000000000");
                },
                // li Rd inm
                Opcode::Li => {
                    let register_id = get_reg_id(&mut word);
                    if register_id < 16 {
                        let inm_num = get_raw_num(&mut word);
                        bin_instr.push_str(format!("{:08b}{:04b}", inm_num, register_id)
                            .as_str());
                    }
                },
                // jmp inm
                Opcode::Jmp => {
                    let jump_addr = get_raw_num(&mut word);
                    if jump_addr < 1024 {
                        bin_instr.push_str(format!("{:010b}", jump_addr).as_str());
                    }
                },
                // jz inm
                Opcode::Jz => {
                    let jump_addr = get_raw_num(&mut word);
                    if jump_addr < 1024 {
                        bin_instr.push_str(format!("{:010b}", jump_addr).as_str());
                    }
                },
                // jnz inm
                Opcode::Jnz => {
                    let jump_addr = get_raw_num(&mut word);
                    if jump_addr < 1024 {
                        bin_instr.push_str(format!("{:010b}", jump_addr).as_str());
                    }
                },
                // mov Rd, Rs
                Opcode::Mov => {
                    let rd = get_reg_id(&mut word);
                    let rs = get_reg_id(&mut word);
                    bin_instr.push_str(format!("{:04b}{:04b}{:04b}",
                        rs, 0, rd).as_str());
                },
                // not Rs, Rd
                Opcode::Not => {
                    let rd = get_reg_id(&mut word);
                    let rs = get_reg_id(&mut word);
                    bin_instr.push_str(format!("{:04b}{:04b}{:04b}",
                        rs, 0, rd).as_str());
                },
                // add Rd, R1, R2
                Opcode::Add => {
                    let rd = get_reg_id(&mut word);
                    let r1 = get_reg_id(&mut word);
                    let r2 = get_reg_id(&mut word);
                    bin_instr.push_str(format!("{:04b}{:04b}{:04b}",
                        r1, r2, rd).as_str());
                },
                // sub Rd, R1, R2
                Opcode::Sub => {
                    let rd = get_reg_id(&mut word);
                    let r1 = get_reg_id(&mut word);
                    let r2 = get_reg_id(&mut word);
                    bin_instr.push_str(format!("{:04b}{:04b}{:04b}",
                        r1, r2, rd).as_str());
                },
                // and Rd, R1, R2
                Opcode::And => {
                    let rd = get_reg_id(&mut word);
                    let r1 = get_reg_id(&mut word);
                    let r2 = get_reg_id(&mut word);
                    bin_instr.push_str(format!("{:04b}{:04b}{:04b}",
                        r1, r2, rd).as_str());
                },
                // or Rd, R1, R2
                Opcode::Or => {
                    let rd = get_reg_id(&mut word);
                    let r1 = get_reg_id(&mut word);
                    let r2 = get_reg_id(&mut word);
                    bin_instr.push_str(format!("{:04b}{:04b}{:04b}",
                        r1, r2, rd).as_str());
                },
                Opcode::Call => {
                    let jump_addr = get_raw_num(&mut word);
                    if jump_addr < 1024 {
                        bin_instr.push_str(format!("{:010b}", jump_addr).as_str());
                    }
                },
                Opcode::Ret => {
                    bin_instr.push_str("0000000000");
                },
                Opcode::Lw => {
                    let rd = get_reg_id(&mut word);
                    let mem_addr = get_raw_num(&mut word);
                    bin_instr.push_str(format!("{:06b}{:04b}", mem_addr, rd)
                        .as_str());
                },
                Opcode::Sw => {
                    let rs = get_reg_id(&mut word);
                    let mem_addr = get_raw_num(&mut word);
                    bin_instr.push_str(format!("{:04b}{:06b}", rs, mem_addr)
                        .as_str());
                },
                Opcode::In => {
                    let port = get_port_id(&mut word);
                    let mem_addr = get_raw_num(&mut word);
                    bin_instr.push_str(format!("{:04b}{:06b}", port, mem_addr)
                        .as_str());
                },
                Opcode::Out => {
                    let port = get_port_id(&mut word);
                    let mem_addr = get_raw_num(&mut word);
                    bin_instr.push_str(format!("{:04b}{:06b}", port, mem_addr)
                        .as_str());
                },
            }
            // println!("{}", bin_instr);
            bin_vec.push(bin_instr);
        }
    }

    bin_vec
}

fn get_reg_id(word: &mut std::slice::Iter<'_, std::string::String>) -> u8 {
    word.next().unwrap().split_terminator("R")
        .collect::<String>().parse::<u8>().unwrap()
}

fn get_port_id(word: &mut std::slice::Iter<'_, std::string::String>) -> u8 {
    word.next().unwrap().split_terminator("P")
        .collect::<String>().parse::<u8>().unwrap()
}

fn get_raw_num(word: &mut std::slice::Iter<'_, std::string::String>) -> u16 {
    word.next().unwrap().parse::<u16>().unwrap()
}

fn add_underscore_padding(bin_instr: &mut Vec<String>) {
    for line in bin_instr {
        let initial_len = line.len();
        let mut expansion = 0;
        for pos in 1..initial_len / 4 {
            line.insert(pos * 4 + expansion,   '_');
            expansion += 1;
        }
    }
}

fn capitalize_str(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() -> Result<(), io::Error> {
    let arg_parsing = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .arg(Arg::with_name("FILE")
             .required_unless("version"))
        .arg(Arg::with_name("outfile")
             .short("o")
             .long("outfile")
             .takes_value(true)
             .help("Specifies the name for the machine code file.\n\
                    Default filename is \"a.out\""))
        .arg(Arg::with_name("version")
             .short("v")
             .long("version")
             .takes_value(false)
             .help("Shows the version number."))
        .get_matches();

    let infile_arg = arg_parsing.value_of("FILE");

    if let Some(infile) = infile_arg {
        let outfile_arg = arg_parsing.value_of("outfile");
        parse_machine_code(infile, outfile_arg)?;
    }
    else if arg_parsing.is_present("version") {
        println!("{} {}", clap::crate_name!(), 
                 clap::crate_version!());
    }

    Ok(())
}
