use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about="qwikey", version="0.1.0", author="0xDAWS")]
pub struct Args {
    // Output Length
    #[clap(short='L', 
           long="length", 
           help_heading="REQUIRED",
           help="Length of string to output")]
    pub length: usize,

    #[clap(short, 
           long, 
           help="Use the ASCII uppercase charset")]
    pub upper: bool,

    #[clap(short, 
           long, 
           help="Use the ASCII uppercase charset")]
    pub lower: bool,

    #[clap(short, 
           long, 
           help="Use the ASCII digits charset")]
    pub digits: bool,

    #[clap(short, 
           long, 
           help="Use the ASCII special characters charset")]
    pub special: bool,

    #[clap(short='n', 
           long="no-lookalike", 
           help="Remove all characters which may be hard to differentiate in certain fonts [0,O,o,l,1,i,I,|,!] from random pool")]
    pub noambiguous: bool,

    #[clap(short='x',long="hex",help="Create a random string of hex characters (lowercase)")]
    pub hex: bool,

    #[clap(short='X',long="hex-upper",help="Create a random string of hex characters (uppercase)")]
    pub hexupper: bool,

    #[clap(short='e',long="entropy",help="Print the entropy of the generated key in bits")]
    pub entropy: bool
}

pub fn get_args() -> Args {
    return Args::parse();
}