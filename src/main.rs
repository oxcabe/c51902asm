fn help() {
    println!("Usage: c51902asm [OPTIONS]... [INFILE]...
Assembler for the 8-bit c51902 architecture.

-h, --help      shows this menu.
-o, --outfile   specifies the name for the machine code file.
                Default filename is \"a.out\".
-v, --version   shows the version number.");
}

fn main() {
    help();
}
