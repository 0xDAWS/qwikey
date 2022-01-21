mod argparser;
mod rng;
mod utils;

use rng::{get_str_key,get_hex_key};
use argparser::Args;

enum SetMode {
    HexMode,
    StrMode,
}

impl SetMode {
    fn run(&mut self, args: &Args) {
        match self {
            Self::HexMode => get_hex_key(args),
            Self::StrMode => get_str_key(args), 
        }
    }
}


fn main() {
    let args: Args = argparser::get_args();

    if let true = (args.hexlower || args.hexupper) {
        let mut mode = SetMode::HexMode;
        mode.run(&args);

    } else {
        let mut mode = SetMode::StrMode;
        mode.run(&args);
    }
}
