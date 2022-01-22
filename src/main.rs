mod argparser;
mod rng;
mod utils;

use argparser::Args;
use rng::{get_hex_key, get_str_key};

enum SetMode {
    HexMode,
    StrMode,
}

impl SetMode {
    fn run(&self, args: &Args) {
        match self {
            Self::HexMode => get_hex_key(args),
            Self::StrMode => get_str_key(args),
        }
    }
}

fn main() {
    let args: Args = argparser::get_args();

    if args.hexlower || args.hexupper {
        let mode: SetMode = SetMode::HexMode;
        mode.run(&args);
    } else {
        let mode: SetMode = SetMode::StrMode;
        mode.run(&args);
    }
}
