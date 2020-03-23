extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::io;

fn parse_machine_code(infile: &str, outfile: Option<&str>) -> Result<String, io::Error> {
    let infile_content: String = fs::read_to_string(&infile)?;
    println!("{}:\n{}", infile, infile_content);

    Ok(infile_content)
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
