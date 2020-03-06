extern crate clap;

use clap::{Arg, App};

fn main() {
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
}
