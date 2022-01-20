use clap::Parser;

#[derive(Parser)]
#[clap(about = "qwikey", version = "0.1.0", author = "0xDAWS")]
pub struct Args {
    #[clap(
       short, 
       long, 
       help = "Use the ASCII uppercase charset")]
    pub upper: bool,

    #[clap(
       short, 
       long, 
       help = "Use the ASCII lowercase charset")]
    pub lower: bool,

    #[clap(
       short, 
       long, 
       help = "Use the ASCII digits charset")]
    pub digits: bool,

    #[clap(
       short, 
       long, 
       help = "Use the ASCII special characters charset")]
    pub special: bool,

    #[clap(
       short = 'n',
       long = "no-lookalike",
       help = "Remove all characters which may be hard to differentiate in certain fonts [0,O,o,l,1,i,I,|,!] from random pool"
    )]
    pub noambiguous: bool,

    #[clap(
       short = 'x', 
       long = "hex", 
       help = "Generate a hex string (lowercase)")]
    pub hexlower: bool,

    #[clap(
       short = 'X',
       long = "hex-upper",
       help = "Generate a hex string (uppercase)"
    )]
    pub hexupper: bool,

    #[clap(
       short = 'e',
       long = "entropy",
       help = "Calculate the entropy of the generated key in bits"
    )]
    pub entropy: bool,

    #[clap(
       short = 'L', 
       long = "length", 
       help = "Length of string to output")]
    pub length: usize,
}

pub fn get_args() -> Args {
    return Args::parse();
}
